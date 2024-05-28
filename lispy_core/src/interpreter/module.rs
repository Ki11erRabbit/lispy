use std::collections::HashMap;
use std::sync::Arc;
use std::cell::RefCell;
use crate::interpreter::value::Value;

use super::context::{ContextFrame, Context};


#[derive(Debug, Clone)]
enum RawModule {
    File(String, Vec<String>),
    Loaded {
	submodules: Arc<HashMap<String, Module>>,
	frame: Arc<ContextFrame>,
    },
}

#[derive(Debug, Clone)]
pub struct Module {
    raw_module: RefCell<RawModule>,
}

impl Module {
    pub fn new(path: &str, module_name: Vec<String>) -> Self {
	Module {
	    raw_module: RefCell::new(RawModule::File(path.to_string(), module_name)),
	}
    }

    pub fn new_loaded(submodules: HashMap<String, Module>, frame: ContextFrame) -> Self {
	Module {
	    raw_module: RefCell::new(RawModule::Loaded {
		submodules: Arc::new(submodules),
		frame: Arc::new(frame),
	    }),
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
	    RawModule::File(path, module_name) => {
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
			let file = crate::parser::parse(&file_content, &mut context.get_macros()).expect("parse error");
			context.push_frame(None);
			crate::interpreter::walkthrough::run(file, &mut context, &module_name).expect("run error");
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
	    RawModule::File(_, _) => unreachable!(),
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
	    RawModule::File(_, _) => {},
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
	    RawModule::File(_, _) => {},
	    RawModule::Loaded { submodules, frame } => {
		for (_, module) in submodules.iter() {
		    module.unmark();
		}
		frame.unmark();
	    }
	}
    }

    pub fn get_submodule(&self, path: &[String], context: &Context) -> Option<Module> {
	self.load(context);
	match &*self.raw_module.borrow() {
	    RawModule::File(_, _) => unreachable!(),
	    RawModule::Loaded { submodules, .. } => {
		if path.is_empty() {
		    return Some(self.clone());
		} else if path.len() == 1 {
		    match submodules.get(&path[0]).cloned() {
			None => None,
			Some(module) => module.get_submodule(&[], context),
		    }
		} else {
		    if let Some(module) = submodules.get(&path[0]) {
			module.get_submodule(&path[1..], context)
		    } else {
			None
		    }
		}
	    }
	}
    }

    pub fn into_loaded(self) -> Option<(HashMap<String, Module>, ContextFrame)> {
	match self.raw_module.into_inner() {
	    RawModule::File(_, _) => None,
	    RawModule::Loaded { submodules, frame } => Some(((*submodules).clone(), (*frame).clone())),
	}
    }
		    
}

// FFI functions
impl Module {
    #[no_mangle]
    pub extern "C" fn module_new_loaded(frame: *mut ContextFrame) -> *mut Module {
	let frame = unsafe { Box::from_raw(frame) };
	let submodules = HashMap::new();
	let module = Module::new_loaded(submodules, *frame);
	Box::into_raw(Box::new(module))
    }

}
