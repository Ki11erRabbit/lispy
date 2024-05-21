use std::collections::{HashMap, HashSet}; 
use crate::gc::{self, Gc};
use crate::interpreter::value::Value;
use crate::interpreter::module::Module;
use crate::stdlib::get_stdlib;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use std::cell::RefCell;

use super::value::GcValue;

#[derive(Clone)]
pub struct ContextFrame {
    pub bindings: HashMap<String, Value>,
}

impl ContextFrame {
    pub fn new() -> Self {
	ContextFrame {
	    bindings: HashMap::new(),
	}
    }
    pub fn new_with_bindings(bindings: HashMap<String, Value>) -> Self {
	ContextFrame {
	    bindings,
	}
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
	    let value = self.bindings.get(name);// this blocks the thread
        value
    }

    pub fn mark(&self) {
	for (_, value) in self.bindings.iter() {
	    value.mark();
	}
    }

    pub fn unmark(&self) {
	for (_, value) in self.bindings.iter() {
	    value.unmark();
	}
    }

    pub fn protect(&self) {
	for (_, value) in self.bindings.iter() {
	    value.protect();
	}
    }
}

pub struct Context {
    gc_lock: Arc<RwLock<()>>,
    sender: Sender<Gc<GcValue>>,
    modules: RefCell<HashMap<String, Module>>,
    frames: Vec<ContextFrame>,
}

impl Context {
    pub fn new(gc_lock: Arc<RwLock<()>>, sender: Sender<Gc<GcValue>>) -> Self {
	let mut ctx = Context {
	    gc_lock,
	    sender,
	    modules: RefCell::new(HashMap::new()),
	    frames: vec![],
	};
	let stdlib = get_stdlib(&mut ctx);
	ctx.frames.push(stdlib);

	let thread = crate::stdlib::thread::get_thread_library(&mut ctx);
	ctx.add_module("thread", thread);
	ctx
    }

    pub fn push_frame(&mut self, frame: Option<ContextFrame>) {
	match frame {
	    Some(frame) => self.frames.push(frame),
	    None => self.frames.push(ContextFrame { bindings: HashMap::new() }),
	}
    }

    pub fn pop_frame(&mut self) -> Option<ContextFrame> {
	self.frames.pop()
    }

    pub fn copy_frame(&mut self) -> ContextFrame {
	let frame = self.frames.last().unwrap().clone();
	frame
    }

    pub fn copy_frame_at(&self, pos: usize) -> ContextFrame {
	self.frames[pos].clone()
    }

    fn get_from_frame(&self, name: &str) -> Option<&Value> {
        for frame in self.frames.iter().rev() {
            if let Some(value) = frame.get(name) {
                return Some(value);
            }
        }
    	None
    }
    

    pub fn get(&self, name: &Vec<String>) -> Option<Value> {
        if name.len() == 1 {
            let value = self.get_from_frame(&name[0]);
            return value.cloned();
        }
        if let Some(module) = self.modules.borrow_mut().get_mut(&name[0]) {
            module.get(&name.as_slice()[1..], self)
        } else {
            None
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
	self.frames.last_mut().unwrap().bindings.insert(name.to_string(), value);
    }

    pub fn add_module(&mut self, name: &str, module: Module) {
	self.modules.borrow_mut().insert(name.to_string(), module);
    }
    
    pub fn garbage_collect(&mut self) {
	let lock = self.gc_lock.read().unwrap();
	for frame in self.frames.iter_mut() {
	    frame.mark();
	}
	for (_, module) in self.modules.borrow_mut().iter_mut() {
	    module.mark();
	}
	drop(lock);
	while gc::is_gc_on() {

	}
	for frame in self.frames.iter_mut() {
	    frame.unmark();
	}
	for (_, module) in self.modules.borrow_mut().iter_mut() {
	    module.unmark();
	}
	
    }

    pub fn send_gc(&self, gc: Gc<GcValue>) {
	self.sender.send(gc).unwrap();
    }

    pub fn module_list(&self) -> HashSet<String> {
	self.modules.borrow().keys().cloned().collect()
    }

    pub fn remove_module(&mut self, name: &str) -> Option<Module> {
	self.modules.borrow_mut().remove(name)
    }

    pub fn pop_modules(&mut self) -> HashMap<String, Module> {
	self.modules.borrow_mut().drain().collect()
    }

    pub fn add_modules(&mut self, modules: HashMap<String, Module>) {
	for (name, module) in modules {
	    self.modules.borrow_mut().insert(name, module);
	}
    }
	
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Clone for Context {
    fn clone(&self) -> Self {
	Context {
	    gc_lock: self.gc_lock.clone(),
	    sender: self.sender.clone(),
	    modules: RefCell::new(HashMap::new()),
	    frames: Vec::new(),
	}
    }
}

impl Default for Context {
    fn default() -> Self {
	let (sender, _) = std::sync::mpsc::channel();
	Context::new(Arc::new(RwLock::new(())), sender)
    }
}
