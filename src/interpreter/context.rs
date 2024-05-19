use std::collections::HashMap; 


pub struct ContextFrame {
    pub bindings: HashMap<String, Value>,
}

pub struct Context {
    modules: HashMap<String, Module>,
    frames: Vec<ContextFrame>,
}

impl Context {
    pub fn new() -> Self {
	Context {
	    modules: HashMap::new(),
	    frames: vec![ContextFrame { bindings: HashMap::new() }],
	}
    }

    pub fn push_frame(&mut self) {
	self.frames.push(ContextFrame { bindings: HashMap::new() });
    }

    pub fn pop_frame(&mut self) {
	self.frames.pop();
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
	for frame in self.frames.iter().rev() {
	    if let Some(value) = frame.bindings.get(name) {
		return Some(value);
	    }
	}
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
	for frame in self.frames.iter_mut().rev() {
	    if let Some(value) = frame.bindings.get_mut(name) {
		return Some(value);
	    }
	}
    }

    pub fn define(&mut self, name: &str, value: Value) {
	self.frames.last_mut().unwrap().bindings.insert(name.to_string(), value);
    }
	

}
