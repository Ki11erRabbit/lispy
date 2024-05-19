use std::collections::HashMap;

use crate::interpreter::value::Value;
use crate::parser::File;

use super::context::ContextFrame;




pub struct Module {
    raw_module: RawModule,
}

impl Module {
    pub fn new(file: File) -> Self {
	Module {
	    raw_module: RawModule::File(file),
	}
    }

    #[inline]
    fn load(&mut self) {
	match &mut self.raw_module {
	    RawModule::File(file) => {
		todo!("load module");
	    }
	    RawModule::Loaded { .. } => {}
	}
    }

    pub fn get(&mut self, path: &[String]) -> Option<&Value> {
	self.load();
	if path.is_empty() {
	    return None;
	}
	match &mut self.raw_module {
	    RawModule::File(_) => unreachable!(),
	    RawModule::Loaded { submodules, frame } => {
		if path.len() == 1 {
		    return frame.get(&path[0]);
		}
		if let Some(module) = submodules.get_mut(&path[0]) {
		    module.get(&path[1..])
		} else {
		    None
		}
	    }
	}
    }
}

enum RawModule {
    File(File),
    Loaded {
	submodules: HashMap<String, Module>,
	frame: ContextFrame,
    },
}
