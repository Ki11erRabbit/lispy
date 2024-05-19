use std::collections::HashMap; 
use crate::interpreter::value::Value;
use crate::interpreter::module::Module;
use crate::stdlib::get_stdlib;

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
    
}

pub struct Context {
    modules: HashMap<String, Module>,
    frames: Vec<ContextFrame>,
}

impl Context {
    pub fn new() -> Self {
	Context {
	    modules: HashMap::new(),
	    frames: vec![get_stdlib()],
	}
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
    

    pub fn get(&mut self, name: &Vec<String>) -> Option<&Value> {
	if name.len() == 1 {
	    return self.get_from_frame(&name[0]);
	}
	if let Some(module) = self.modules.get_mut(&name[0]) {
	    module.get(&name.as_slice()[1..])
	} else {
	    None
	}
    }

    pub fn define(&mut self, name: &str, value: Value) {
	self.frames.last_mut().unwrap().bindings.insert(name.to_string(), value);
    }

    pub fn add_module(&mut self, name: &str, module: Module) {
	self.modules.insert(name.to_string(), module);
    }
    

}
