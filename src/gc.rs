use std::sync::mpsc::TryRecvError;
use std::sync::{Mutex, RwLock, Arc};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, AtomicBool};
use crate::interpreter::context::Context;
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

pub struct Gc<T: ?Sized> {
    marked: NonNull<Mutex<Mark>>,
    ptr: NonNull<GcBox<T>>,
}

impl<T> Gc<T> {
    pub fn new(value: T) -> Self {
	let gc = Gc {
	    marked: NonNull::new(Box::into_raw(Box::new(Mutex::new(Mark::White)))).unwrap(),
	    ptr: NonNull::new(Box::into_raw(Box::new(GcBox { count: 1.into(), value }))).unwrap(),
	};
	gc
    }
}

impl<T: ?Sized> Gc<T> {

    pub fn mark(&mut self) {
	let mut current = unsafe {
	    self.marked.as_ref().lock().unwrap()
	};
	match *current {
	    Mark::White => *current = Mark::Gray,
	    Mark::Gray => *current = Mark::Black,
	    Mark::Black => {},
	}
    }

    pub fn unmark(&mut self) {
	unsafe {
	    *self.marked.as_ref().lock().unwrap() = Mark::White;
	}
    }
	

    pub fn marked(&self) -> Mark {
	unsafe {
	    *self.marked.as_ref().lock().unwrap()
	}
    }

    pub fn get(&self) -> &T {
	unsafe {
	    &self.ptr.as_ref().value
	}
    }

    pub fn get_mut(&mut self) -> &mut T {
	unsafe {
	    &mut self.ptr.as_mut().value
	}
    }
}

unsafe impl<T> Send for Gc<T> {}
unsafe impl<T> Sync for Gc<T> {}

impl<T> Clone for Gc<T> {
    fn clone(&self) -> Self {
	unsafe {
	    let ptr = self.ptr.as_ref();
	    ptr.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
	    let marked = self.marked;
	    Gc {
		marked,
		ptr: self.ptr,
	    }
	}
    }
}

impl<T: ?Sized> Drop for Gc<T> {
    fn drop(&mut self) {
	unsafe {
	    let ptr = self.ptr.as_ref();
	    ptr.count.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
	    if ptr.count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
		println!("dropping");
		drop(Box::from_raw(self.ptr.as_ptr()));
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
		    println!("removing {}", i);
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
