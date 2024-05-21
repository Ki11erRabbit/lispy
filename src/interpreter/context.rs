use std::collections::HashMap; 
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
	self.bindings.get(name)
    }

    pub fn mark(&mut self) {
	for (_, value) in self.bindings.iter_mut() {
	    value.mark();
	}
    }

    pub fn unmark(&mut self) {
	for (_, value) in self.bindings.iter_mut() {
	    value.unmark();
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

    fn get_from_frame(&self, name: &str) -> Option<&Value> {
	for frame in self.frames.iter().rev() {
	    if let Some(value) = frame.bindings.get(name) {
		return Some(value);
	    }
	}
	None
    }
    

    pub fn get(&self, name: &Vec<String>) -> Option<Value> {
	if name.len() == 1 {
	    return self.get_from_frame(&name[0]).cloned();
	}
	if let Some(module) = self.modules.borrow_mut().get_mut(&name[0]) {
	    module.get(&name.as_slice()[1..]).cloned()
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
}

impl Default for Context {
    fn default() -> Self {
	let (sender, _) = std::sync::mpsc::channel();
	Context::new(Arc::new(RwLock::new(())), sender)
    }
}
