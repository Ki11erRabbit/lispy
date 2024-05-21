use std::collections::HashMap;
use std::sync::Arc;
use std::cell::RefCell;
use crate::interpreter::value::Value;

use super::context::{ContextFrame, Context};



#[derive(Clone)]
pub struct Module {
    raw_module: RefCell<RawModule>,
}

impl Module {
    pub fn new(path: &str) -> Self {
	Module {
	    raw_module: RefCell::new(RawModule::File(path.to_string())),
	}
    }

    pub fn new_from_context(mut context: Context) -> Self {
	Module {
	    raw_module: RefCell::new(RawModule::Loaded {
		submodules: Arc::new(HashMap::new()),
		frame: Arc::new(context.pop_frame().expect("pop error")),
	    }),
	}
    }

    #[inline]
    fn load(&self, context: &Context) {
	let mut new_self = None;
	match &mut *self.raw_module.borrow_mut() {
	    RawModule::File(path) => {
		let mut context = context.clone();
		let load_path = std::env::var("LISPY_LOAD_PATH").unwrap_or(".".to_string());
		let load_path = if load_path.as_str() != "." {
		    load_path + ":."
		} else {
		    load_path
		};
		let load_path = load_path.split(":").collect::<Vec<_>>();
		for dir in load_path {
		    let full_path = format!("{}/{}", dir, path);
		    if let Ok(file_content) = std::fs::read_to_string(&full_path) {
			let file = crate::parser::parse(&file_content).expect("parse error");
			context.push_frame(None);
			crate::interpreter::walk_through::run(file, &mut context).expect("run error");
			let frame = context.pop_frame().expect("pop error");
			let submodules = context.pop_modules();
			let frame = Arc::new(frame);
			let submodules = Arc::new(submodules);
			new_self = Some(RawModule::Loaded {
			    submodules,
			    frame,
			});
		    }
		}
	    }
	    RawModule::Loaded { .. } => {}
	}
	if let Some(new_self) = new_self {
	    *self.raw_module.borrow_mut() = new_self;
	}
    }

    pub fn get(&self, path: &[String], context: &Context) -> Option<Value> {
	self.load(context);
	if path.is_empty() {
	    return None;
	}
	match &*self.raw_module.borrow() {
	    RawModule::File(_) => unreachable!(),
	    RawModule::Loaded { submodules, frame } => {
		if path.len() == 1 {
		    return frame.get(&path[0]).cloned();
		}
		if let Some(module) = submodules.get(&path[0]) {
		    module.get(&path[1..], context)
		} else {
		    None
		}
	    }
	}
    }

    pub fn mark(&self) {
	match &*self.raw_module.borrow() {
	    RawModule::File(_) => {},
	    RawModule::Loaded { submodules, frame } => {
		for (_, module) in submodules.iter() {
		    module.mark();
		}
		frame.mark();
	    }
	}
    }

    pub fn unmark(&self) {
	match &*self.raw_module.borrow() {
	    RawModule::File(_) => {},
	    RawModule::Loaded { submodules, frame } => {
		for (_, module) in submodules.iter() {
		    module.unmark();
		}
		frame.unmark();
	    }
	}
    }
		    
}

#[derive(Clone)]
enum RawModule {
    File(String),
    Loaded {
	submodules: Arc<HashMap<String, Module>>,
	frame: Arc<ContextFrame>,
    },
}
