use std::sync::mpsc::TryRecvError;
use std::sync::{Mutex, RwLock, Arc};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, AtomicBool};
use crate::interpreter::value::GcValue;

static mut GC_ON: AtomicBool = AtomicBool::new(false);

pub fn is_gc_on() -> bool {
    unsafe {
	GC_ON.load(std::sync::atomic::Ordering::Relaxed)
    }
}

fn set_gc_on() {
    unsafe {
	GC_ON.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

fn set_gc_off() {
    unsafe {
	GC_ON.store(false, std::sync::atomic::Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    White,
    Gray,
    Black,
}

#[repr(C)]
struct GcBox<T: ?Sized> {
    count: AtomicUsize,
    value: T,
}

pub enum Gc<T: ?Sized> {
    Normal {
	marked: NonNull<Mutex<Mark>>,
	ptr: NonNull<GcBox<T>>,
    },
    Protected {
	marked: NonNull<Mutex<Mark>>,
	ptr: Mutex<NonNull<GcBox<T>>>,
    },
}


impl<T> Gc<T> {
    pub fn new(value: T) -> Self {
	let gc = Gc::Normal {
	    marked: NonNull::new(Box::into_raw(Box::new(Mutex::new(Mark::White)))).unwrap(),
	    ptr: NonNull::new(Box::into_raw(Box::new(GcBox { count: 1.into(), value }))).unwrap(),
	};
	gc
    }
}

impl<T: ?Sized> Gc<T> {

    pub fn mark(&self) {
	let mut current = unsafe {
	    match self {
		Gc::Normal { marked, .. } => marked.as_ref().lock().unwrap(),
		Gc::Protected { marked, .. } => marked.as_ref().lock().unwrap(),
	    }
	};
	match *current {
	    Mark::White => *current = Mark::Gray,
	    Mark::Gray => *current = Mark::Black,
	    Mark::Black => {},
	}
    }

    pub fn unmark(&self) {
	unsafe {
	    match self {
		Gc::Normal { marked, .. } => *marked.as_ref().lock().unwrap() = Mark::White,
		Gc::Protected { marked, .. } => *marked.as_ref().lock().unwrap() = Mark::White,
	    }
	}
    }
	

    pub fn marked(&self) -> Mark {
	unsafe {
	    match self {
		Gc::Normal { marked, .. } => *marked.as_ref().lock().unwrap(),
		Gc::Protected { marked, .. } => *marked.as_ref().lock().unwrap(),
	    }
	}
    }

    pub fn get(&self) -> &T {
	unsafe {
	    match self {
		Gc::Normal { ptr, .. } => &ptr.as_ref().value,
		Gc::Protected { ptr, .. } => &ptr.lock().unwrap().as_mut().value,
	    }
	}
    }

    pub fn get_mut(&mut self) -> &mut T {
	unsafe {
	    match self {
		Gc::Normal { ptr, .. } => &mut ptr.as_mut().value,
		Gc::Protected { ptr, .. } => &mut ptr.lock().unwrap().as_mut().value,
	    }
	}
    }

    pub fn protect(&self) {
	unsafe {
	    // TODO: Change to use UnsafeCell
	    #[allow(mutable_transmutes)]
	    let this = std::mem::transmute::<&Gc<T>, &mut Gc<T>>(self);
	    match self {
		Gc::Normal { marked, ptr } => {
		    *this = Gc::Protected {
			ptr: Mutex::new(*ptr),
			marked: *marked,
		    };
		},
		Gc::Protected { .. } => {},
	    }
	}
    }
}

unsafe impl<T> Send for Gc<T> {}
unsafe impl<T> Sync for Gc<T> {}

impl<T> Clone for Gc<T> {
    fn clone(&self) -> Self {
	unsafe {
	    match self {
		Gc::Normal { ptr, marked } => {
		    let ptr_ref = ptr.as_ref();
		    ptr_ref.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
		    Gc::Normal {
			ptr: *ptr,
			marked: *marked,
		    }
		},
		Gc::Protected { ptr, marked } => {
		    let ptr_ref = ptr.lock().unwrap();
		    ptr_ref.as_ref().count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

		    Gc::Protected {
			ptr: std::ptr::read(ptr),
			marked: *marked,
		    }
		},
	    }
	}
    }
}

impl<T: ?Sized> Drop for Gc<T> {
    fn drop(&mut self) {
	unsafe {
	    match self {
		Gc::Normal { ptr, .. } => {
		    let ptr_ref = ptr.as_ref();
		    ptr_ref.count.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
		    if ptr_ref.count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
			drop(Box::from_raw(ptr.as_ptr()));
		    }
		},
		Gc::Protected { ptr, .. } => {
		    let ptr_ref = ptr.lock().unwrap();
		    ptr_ref.as_ref().count.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
		    if ptr_ref.as_ref().count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
			let ptr = ptr.lock().unwrap();
			drop(Box::from_raw(ptr.as_ptr()));
		    }
		},
	    }
	}
    }
}

pub struct GcTable {
    gc_lock: Arc<RwLock<()>>,
    receiver: std::sync::mpsc::Receiver<Gc<GcValue>>,
    table: Vec<Gc<GcValue>>,
}

impl GcTable {
    pub fn new(gc_lock: Arc<RwLock<()>>, receiver: std::sync::mpsc::Receiver<Gc<GcValue>>) -> Self {
	GcTable {
	    gc_lock,
	    receiver,
	    table: Vec::new(),
	}
    }

    pub fn insert(&mut self, value: Gc<GcValue>) {
	self.table.push(value);
    }

    pub fn garbage_collect(&mut self) {
	let mut remove_indices = Vec::new();
	for (i, value) in self.table.iter_mut().enumerate() {
	    match value.marked() {
		Mark::White => {
		    remove_indices.push(i);
		},
		_ => {},
	    }
	}
	for i in remove_indices.iter().rev() {
	    self.table.remove(*i);
	}
    }
}


pub fn garbage_collect(table: &mut GcTable) {
    let mut time = std::time::Instant::now();
    loop {
	match table.receiver.try_recv() {
	    Ok(value) => {
		table.insert(value);
	    },
	    Err(TryRecvError::Empty) => {},
	    Err(TryRecvError::Disconnected) => break,
	}
	if time.elapsed().as_secs() < 300 {
	    continue;
	}
	
	set_gc_on();
	std::thread::sleep(std::time::Duration::from_secs(1));
	let lock = table.gc_lock.clone();
	let _guard = lock.write().unwrap();
	table.garbage_collect();
	set_gc_off();
	time = std::time::Instant::now();
    }
}
