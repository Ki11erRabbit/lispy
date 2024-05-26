use std::ffi::CString;
use crate::parser::Atom;
use crate::parser::Sexpr;
use crate::interpreter::{self, HelperResult};

use crate::interpreter::bytecode::Bytecode;
use crate::interpreter::kwargs::Kwargs;
use crate::interpreter::{context::{ContextFrame, Context}, Exception, InterpreterResult};
use crate::interpreter::value::Value;

#[repr(C)]
pub enum CFunctionOutput {
    Value(Value),
    Exception(Exception),
    Blank,
}

impl CFunctionOutput {
    #[no_mangle]
    pub extern "C" fn set_return_value(&mut self, value: *mut Value) {
	unsafe {
	    let value = Box::from_raw(value);
	    *self = CFunctionOutput::Value(*value.clone());
	}
    }
    #[no_mangle]
    pub extern "C" fn set_exception_value(&mut self, exception: *mut Exception) {
	unsafe {
	    let exception = Box::from_raw(exception);
	    *self = CFunctionOutput::Exception(*exception.clone());
	}
    }
}


#[derive(Clone)]
pub enum Function {
    Tree(Vec<String>, Sexpr, ContextFrame, FunctionShape),
    Native(fn(&mut Context, Vec<Value>, Kwargs) -> HelperResult<Value>, FunctionShape),
    Bytecode(Vec<String>, Vec<Bytecode>, FunctionShape),
    CNative(unsafe extern "C" fn(*mut Context, *mut Value, usize, *mut Kwargs, *mut CFunctionOutput), FunctionShape),
}

impl Function {
    pub fn protect(&self) {
	match self {
	    Function::Tree(_, _, frame, _) => {
		frame.protect();
	    },
	    _ => {},
	}
    }

    pub fn call_raw(&self, args: Vec<Value>, kargs: Kwargs, context: &mut Context, module_name: &Vec<String>) -> InterpreterResult {
	match self {
	    Function::Tree(fun_args, body, frame, _) => {
		let frame = frame.clone();
		context.push_frame(Some(frame));
		for (arg, value) in fun_args.iter().zip(args.iter()) {
		    context.define(arg, value.clone());
		}
		for (arg, value) in kargs.iter() {
		    context.define(arg, value.clone());
		}
		let new_module_name = module_name.clone().into_iter().rev().skip(1).rev().collect();
		let value = interpreter::walkthrough::walk_through(&body, context, &new_module_name);
		context.pop_frame();
		value
	    },
	    Function::Native(f, _) => {
		Ok(Some(f(context, args, kargs)?))
	    },
	    Function::Bytecode(fun_args, bytecode, _) => {
		let frame = ContextFrame::new();
		context.push_frame(Some(frame));
		for (arg, value) in fun_args.iter().zip(args.iter()) {
		    context.define(arg, value.clone());
		}
		for (arg, value) in kargs.iter() {
		    context.define(arg, value.clone());
		}
		let new_module_name = module_name.clone().into_iter().rev().skip(1).rev().collect();
		let value = interpreter::bytecode::run(&bytecode.as_slice(), context, &new_module_name);
		context.pop_frame();
		value
	    },
	    Function::CNative(f, _) => {
		let mut args = args.clone();
		let mut kargs = kargs.clone();
		let mut context = context.clone();
		let mut output = CFunctionOutput::Blank;
		unsafe {
		    f(&mut context, args.as_mut_ptr(), args.len(), &mut kargs, &mut output);
		};

		match output {
		    CFunctionOutput::Value(value) => Ok(Some(value)),
		    CFunctionOutput::Exception(exception) => Err(Box::new(exception)),
		    CFunctionOutput::Blank => {
			let empty: Vec<&str> = Vec::new();
			Err(Box::new(Exception::new(&empty, "C function didn't return a value", &context)))
		    },
		}

	    },
	}
    }
    
    pub fn call(&self, name: &Vec<String>, list: &[Sexpr], context: &mut Context, module_name: &Vec<String>) -> InterpreterResult {
	match self {
	    Function::Tree(fun_args, body, frame, shape) => {
		let mut args = Vec::new();
		let mut keyword_args = Kwargs::new();
		let mut iterator = list.iter();
		while let Some(sexpr) = iterator.next() {
		    match sexpr {
			Sexpr::Atom(Atom::Keyword(k)) => {
			    if let Some(value) = iterator.next() {
				match interpreter::walkthrough::walk_through(value, context, module_name)? {
				    Some(value) => {
					keyword_args.insert(k.clone(), value);
				    }
				    None => {
					return Err(Box::new(Exception::new(&name, "expression didn't result in a value", context)));
				    }
				}
			    } else {
				return Err(Box::new(Exception::new(&name, "unusual syntax", context)));
			    }
			}
			s => {
			    match interpreter::walkthrough::walk_through(s, context, module_name)? {
				Some(value) => {
				    args.push(value);
				}
				None => {
				    return Err(Box::new(Exception::new(&name, "expression didn't result in a value", context)));
				}
			    }
			}
		    }
		}
		shape.check(&name, &args, &keyword_args, context)?;
        
		context.push_frame(Some(frame.clone()));

		for (arg, value) in fun_args.iter().zip(args.iter()) {
		    context.define(arg, value.clone());
		}
		for (arg, value) in keyword_args.iter() {
		    context.define(arg, value.clone());
		}
		let new_module_name = name.clone().into_iter().rev().skip(1).rev().collect();
		let value = interpreter::walkthrough::walk_through(&body, context, &new_module_name);
		context.pop_frame();
		value
	    },
	    Function::Native(f, shape) => {
		let mut args = Vec::new();
		let mut keyword_args = Kwargs::new();
		let mut iterator = list.iter();
		while let Some(sexpr) = iterator.next() {
		    match sexpr {
			Sexpr::Atom(Atom::Keyword(k)) => {
			    if let Some(value) = iterator.next() {
				match interpreter::walkthrough::walk_through(value, context, module_name)? {
				    Some(value) => {
					keyword_args.insert(k.clone(), value);
				    }
				    None => {
					return Err(Box::new(Exception::new(&name, "expression didn't result in a value", context)));
				    }
				}
			    } else {
				return Err(Box::new(Exception::new(&name, "unusual syntax", context)));
			    }
			}
			s => {
			    match interpreter::walkthrough::walk_through(s, context, module_name)? {
				Some(value) => {
				    args.push(value);
				}
				None => {
				    return Err(Box::new(Exception::new(&name, "expression didn't result in a value", context)));
				}
			    }
			}
		    }
		}
		shape.check(&name, &args, &keyword_args, context)?;
		Ok(Some(f(context, args, keyword_args)?))
	    },
	    Function::Bytecode(fun_args, bytecode, shape) => {
		let mut args = Vec::new();
		let mut keyword_args = Kwargs::new();
		let mut iterator = list.iter();
		while let Some(sexpr) = iterator.next() {
		    match sexpr {
			Sexpr::Atom(Atom::Keyword(k)) => {
			    if let Some(value) = iterator.next() {
				match interpreter::walkthrough::walk_through(value, context, module_name)? {
				    Some(value) => {
					keyword_args.insert(k.clone(), value);
				    }
				    None => {
					return Err(Box::new(Exception::new(&name, "expression didn't result in a value", context)));
				    }
				}
			    } else {
				return Err(Box::new(Exception::new(&name, "unusual syntax", context)));
			    }
			}
			s => {
			    match interpreter::walkthrough::walk_through(s, context, module_name)? {
				Some(value) => {
				    args.push(value);
				}
				None => {
				    return Err(Box::new(Exception::new(&name, "expression didn't result in a value", context)));
				}
			    }
			}
		    }
		}
		
		shape.check(&name, &args, &keyword_args, context)?;

		context.push_frame(None);

		for (arg, value) in fun_args.iter().zip(args.iter()) {
		    context.define(arg, value.clone());
		}
		for (arg, value) in keyword_args.iter() {
		    context.define(arg, value.clone());
		}
		
        
		let new_module_name = name.clone().into_iter().rev().skip(1).rev().collect();
		let value = interpreter::bytecode::run(&bytecode.as_slice(), context, &new_module_name);
		context.pop_frame();
		value
	    },
	    Function::CNative(f, shape) => {
		let mut args = Vec::new();
		let mut keyword_args = Kwargs::new();
		let mut iterator = list.iter();
		while let Some(sexpr) = iterator.next() {
		    match sexpr {
			Sexpr::Atom(Atom::Keyword(k)) => {
			    if let Some(value) = iterator.next() {
				match interpreter::walkthrough::walk_through(value, context, module_name)? {
				    Some(value) => {
					keyword_args.insert(k.clone(), value);
				    }
				    None => {
					return Err(Box::new(Exception::new(&name, "expression didn't result in a value", context)));
				    }
				}
			    } else {
				return Err(Box::new(Exception::new(&name, "unusual syntax", context)));
			    }
			}
			s => {
			    match interpreter::walkthrough::walk_through(s, context, module_name)? {
				Some(value) => {
				    args.push(value);
				}
				None => {
				    return Err(Box::new(Exception::new(&name, "expression didn't result in a value", context)));
				}
			    }
			}
		    }
		}
		shape.check(&name, &args, &keyword_args, context)?;

		let mut args = args.clone();
		let mut kargs = keyword_args.clone();
		let mut context = context.clone();
		let mut output = CFunctionOutput::Blank;
		unsafe {
		    f(&mut context, args.as_mut_ptr(), args.len(), &mut kargs, &mut output);
		};

		match output {
		    CFunctionOutput::Value(value) => Ok(Some(value)),
		    CFunctionOutput::Exception(exception) => Err(Box::new(exception)),
		    CFunctionOutput::Blank => {
			let empty: Vec<&str> = Vec::new();
			Err(Box::new(Exception::new(&empty, "C function didn't return a value", &context)))
		    },
		}
	    },
	}
    }

    pub fn call_from_bytecode(&self, name: &Vec<String>, args: Vec<Value>, kargs: Kwargs, context: &mut Context, _: &Vec<String>) -> InterpreterResult {
	match self {
	    Function::Tree(fun_args, body, frame, shape) => {
		context.push_frame(Some(frame.clone()));

		for (arg, value) in fun_args.iter().zip(args.iter()) {
		    context.define(arg, value.clone());
		}
		for (arg, value) in kargs.iter() {
		    context.define(arg, value.clone());
		}

		shape.check(&name, &args, &kargs, context)?;
        
		let new_module_name = name.clone().into_iter().rev().skip(1).rev().collect();
		let value = interpreter::walkthrough::walk_through(&body, context, &new_module_name);
		context.pop_frame();
		value
	    },
	    Function::Native(f, shape) => {
		shape.check(&name, &args, &kargs, context)?;
		Ok(Some(f(context, args, kargs)?))
	    }
	    Function::Bytecode(fun_args, bytecode, shape) => {
		shape.check(&name, &args, &kargs, context)?;

		context.push_frame(None);

		for (arg, value) in fun_args.iter().zip(args.iter()) {
		    context.define(arg, value.clone());
		}
		for (arg, value) in kargs.iter() {
		    context.define(arg, value.clone());
		}

		let new_module_name = name.clone().into_iter().rev().skip(1).rev().collect();
		let value = interpreter::bytecode::run(&bytecode.as_slice(), context, &new_module_name);
		context.pop_frame();
		value
	    },
	    Function::CNative(f, shape) => {
		shape.check(&name, &args, &kargs, context)?;

		let mut args = args.clone();
		let mut kargs = kargs.clone();
		let mut context = context.clone();
		let mut output = CFunctionOutput::Blank;
		unsafe {
		    f(&mut context, args.as_mut_ptr(), args.len(), &mut kargs, &mut output);
		};

		match output {
		    CFunctionOutput::Value(value) => Ok(Some(value)),
		    CFunctionOutput::Exception(exception) => Err(Box::new(exception)),
		    CFunctionOutput::Blank => {
			let empty: Vec<&str> = Vec::new();
			Err(Box::new(Exception::new(&empty, "C function didn't return a value", &context)))
		    },
		}
	    }
	}
    }

}

/*impl Clone for Function {
    fn clone(&self) -> Self {
	match self {
	    Function::Tree(args, body, frame, shape) => {
		Function::Tree(args.clone(), body.clone(), frame.clone(), shape.clone())
	    },
	    Function::Native(f, shape) => {
		Function::Native(*f, shape.clone())
	    },
	    Function::NativeClosure(f, shape) => {
		let raw = Box::into_raw(*f);
		let f = unsafe { Box::from_raw(raw) };
		Function::NativeClosure(f, shape.clone())
	    },
	}
    }
}*/

#[repr(C)]
#[derive(Clone)]
pub struct FunctionShape {
    args: Box<Vec<String>>,
}

impl FunctionShape {
    pub fn new(args: Vec<String>) -> Self {
	FunctionShape {
	    args: Box::new(args),
	}
    }

    pub fn check(&self, name: &Vec<String>, args: &Vec<Value>, keyword_args: &Kwargs, context: &mut Context) -> HelperResult<()> {
	if self.args.len() != args.len() + keyword_args.len() {
	    Err(Box::new(Exception::new(name, "wrong number of arguments", context)))?;
	}

	for (i, arg) in self.args.iter().enumerate() {
	    if i < args.len() {
		continue;
	    } else {
		if !keyword_args.contains_key(arg) {
		    Err(Box::new(Exception::new(name, "invalid keyword", context)))?;
		}
	    }
	}

	Ok(())
    }

    #[no_mangle]
    pub extern "C" fn new_function_shape(args: *mut CString, len: usize) -> FunctionShape {
	let args = unsafe {
	    std::slice::from_raw_parts(args, len)
	};
	let args = args.iter().map(|s| s.to_str().unwrap().to_string()).collect();
	FunctionShape::new(args)
    }
}

impl std::fmt::Display for FunctionShape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	write!(f, "({})", self.args.join(" "))
    }
}
