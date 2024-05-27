use std::ffi::{c_void, c_char};
use std::collections::HashMap;

use crate::interpreter::kwargs::Kwargs;
use crate::interpreter::module::Module;
use crate::interpreter::context::Context;
use crate::interpreter::value::Value;
use crate::interpreter::value::function::{Function, CFunctionOutput, FunctionShape};
use libloading;


#[link(name = "lispy_core")]
extern "C" {
    ///pub fn load_module(context: *mut Bindings);
    pub fn lispy_load_module(context: *mut c_void);
}

struct Bindings {
    bindings: Vec<(String, String, FunctionShape)>,
}

impl Bindings {
    pub fn new() -> Self {
	Bindings {
	    bindings: Vec::new(),
	}
    }

    #[no_mangle]
    pub extern "C" fn bindings_add_binding(&mut self, name: *const c_char, name_len: usize, binding: *const c_char, value_len: usize, shape: *mut FunctionShape) {
	let mut name_buf = Vec::new();
	for i in 0..name_len {
	    let c = unsafe { *name.offset(i as isize) };
	    name_buf.push(c as u8);
	}
	let name = String::from_utf8(name_buf).unwrap();

	let mut binding_buf = Vec::new();
	for i in 0..value_len {
	    let c = unsafe { *binding.offset(i as isize) };
	    binding_buf.push(c as u8);
	}
	let binding = String::from_utf8(binding_buf).unwrap();
	let shape = unsafe { Box::from_raw(shape) };
	self.bindings.push((name, binding, *shape));
    }
}


pub fn load_dynamic_libs(context: &mut Context, module_name: &str, load_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
	for path in std::fs::read_dir(load_path)? {
	    let path = path?;
	    let path = path.path();
	    let path = path.to_str().unwrap();
	    if !path.ends_with(".so") || !path.ends_with(".dll") || !path.ends_with(".dylib") {
		continue;
	    }
	    let lib = libloading::Library::new(path)?;
	    let load_module = lib.get::<unsafe extern "C" fn(*mut Bindings)>(b"lispy_load_module")?;

	    let sub_modules = HashMap::new();
	    context.push_frame(None);

	    let mut bindings = Bindings::new();
	    load_module(&mut bindings);

	    for (name, binding, shape) in bindings.bindings {
		let fun = lib.get::<unsafe extern "C" fn(*mut Context, *mut Value, usize, *mut Kwargs, *mut CFunctionOutput)>(name.as_bytes())?;

		let function = Function::CNative(*fun, shape);
		let function = Value::new_function(function, context);
		context.define(&binding, function);
		
	    }
	    
	    let frame = context.pop_frame().ok_or("frame not found")?;
	    let module = Module::new_loaded(sub_modules, frame);
	    context.add_module(module_name, module);
	    context.add_dynamic_lib(lib);
	}
    }
    Ok(())
}

pub fn load_dynamic_lib(context: &mut Context, module_name: &str, load_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
	let path = load_path;
	let lib = libloading::Library::new(path)?;
	let load_module = lib.get::<unsafe extern "C" fn(*mut Bindings)>(b"lispy_load_module")?;

	let sub_modules = HashMap::new();
	context.push_frame(None);

	let mut bindings = Bindings::new();
	load_module(&mut bindings);

	for (name, binding, shape) in bindings.bindings {
	    let fun = lib.get::<unsafe extern "C" fn(*mut Context, *mut Value, usize, *mut Kwargs, *mut CFunctionOutput)>(name.as_bytes())?;

	    let function = Function::CNative(*fun, shape);
	    let function = Value::new_function(function, context);
	    context.define(&binding, function);

	}

	let frame = context.pop_frame().ok_or("frame not found")?;
	let module = Module::new_loaded(sub_modules, frame);
	context.add_module(module_name, module);
	context.add_dynamic_lib(lib);
    }
    Ok(())
}
