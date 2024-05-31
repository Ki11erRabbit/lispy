use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use crate::interpreter::value::Value;

use super::context::{ContextFrame, Context};


#[derive(Debug, Clone)]
enum RawModule {
    File(String, Vec<String>),
    Loaded {
	frame: Arc<ContextFrame>,
    },
}

#[derive(Debug, Clone)]
pub struct Module {
    raw_module: RefCell<RawModule>,
}

impl Module {
    pub fn new(path: &str, module_path: Vec<String>) -> Self {
	Module {
	    raw_module: RefCell::new(RawModule::File(path.to_string(), module_path)),
	}
    }

    pub fn new_loaded(frame: ContextFrame) -> Self {
	Module {
	    raw_module: RefCell::new(RawModule::Loaded {
		frame: Arc::new(frame),
	    }),
	}
    }

    pub fn new_from_context(mut context: Context) -> Self {
	Module {
	    raw_module: RefCell::new(RawModule::Loaded {
		frame: Arc::new(context.pop_frame().expect("pop error")),
	    }),
	}
    }

    #[inline]
    fn load(&self, context: &Context) {
	let mut new_self = None;
	match &mut *self.raw_module.borrow_mut() {
	    RawModule::File(path, module_path) => {
		if let Ok(file_content) = std::fs::read_to_string(&path) {
		    let mut context = context.clone();
		    let file = crate::parser::parse(&file_content, &mut context.get_macros()).expect("parse error");
		    context.push_frame(None);
		    crate::interpreter::walkthrough::run(file, &mut context, &module_path).expect("run error");
		    let frame = context.pop_frame().expect("pop error");
		    let frame = Arc::new(frame);
		    new_self = Some(RawModule::Loaded {
			frame,
		    });
		}
	    }
	    RawModule::Loaded { .. } => {}
	}
	if let Some(new_self) = new_self {
	    *self.raw_module.borrow_mut() = new_self;
	}
    }

    pub fn get(&self, name: &str, context: &Context) -> Option<Value> {
	self.load(context);
	match &*self.raw_module.borrow() {
	    RawModule::File(_, _) => unreachable!(),
	    RawModule::Loaded { frame } => {
		return frame.get(name).cloned();
	    }
	}
    }

    pub fn mark(&self) {
	match &*self.raw_module.borrow() {
	    RawModule::File(_, _) => {},
	    RawModule::Loaded { frame } => {
		frame.mark();
	    }
	}
    }

    pub fn unmark(&self) {
	match &*self.raw_module.borrow() {
	    RawModule::File(_, _) => {},
	    RawModule::Loaded {  frame } => {
		frame.unmark();
	    }
	}
    }


    pub fn into_loaded(self) -> Option<ContextFrame> {
	match self.raw_module.into_inner() {
	    RawModule::File(_, _) => None,
	    RawModule::Loaded { frame } => Some((*frame).clone()),
	}
    }

}

// FFI functions
impl Module {
    #[no_mangle]
    pub extern "C" fn module_new_loaded(frame: *mut ContextFrame) -> *mut Module {
	let frame = unsafe { Box::from_raw(frame) };
	let module = Module::new_loaded(*frame);
	Box::into_raw(Box::new(module))
    }

}
