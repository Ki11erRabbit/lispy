use rug::Integer;
use std::collections::HashMap;
use crate::parser::Sexpr;

use crate::gc::Gc;

use super::{context::{ContextFrame, Context}, Exception};


pub struct Nil;


#[derive(Clone)]
pub struct Value {
    raw: RawValue, 
}

impl Value {
    pub fn new_string(value: &str, context: &mut Context) -> Self {
	let gc_object = Gc::new(GcValue::String(value.to_string()), context);
	context.send_gc(gc_object.clone());
	let raw = RawValue::Gc(gc_object);
	Value {
	    raw,
	}
    }
    pub fn get_string(&self) -> Result<&String, Box<dyn std::error::Error>> {
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::String(ref s) => Ok(s),
		    _ => Err(Box::new(Exception::new(Vec::new(), "not a string".to_string()))),
		}
	    },
	    _ => Err(Box::new(Exception::new(Vec::new(), "not a string".to_string()))),
	}
    }
    pub fn is_string(&self) -> bool {
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::String(_) => true,
		    _ => false,
		}
	    },
	    _ => false,
	}
    }
    
    pub fn new_integer(value: &str) -> Self {
	Value {
	    raw: RawValue::Integer(Integer::from_str_radix(value, 10).unwrap()),
	}
    }
    pub fn new_integer_from_integer(value: Integer) -> Self {
	Value {
	    raw: RawValue::Integer(value),
	}
    }
    pub fn get_integer(&self) -> Result<&Integer, Box<dyn std::error::Error>> {
	match self.raw {
	    RawValue::Integer(ref i) => Ok(i),
	    _ => Err(Box::new(Exception::new(Vec::new(), "not an integer".to_string()))),
	}
    }
    pub fn is_integer(&self) -> bool {
	match self.raw {
	    RawValue::Integer(_) => true,
	    _ => false,
	}
    }

    pub fn new_float(value: f64) -> Self {
	Value {
	    raw: RawValue::Float(value),
	}
    }
    pub fn get_float(&self) -> Result<f64, Box<dyn std::error::Error>> {
	match self.raw {
	    RawValue::Float(f) => Ok(f),
	    _ => Err(Box::new(Exception::new(Vec::new(), "not a float".to_string()))),
	}
    }
    pub fn is_float(&self) -> bool {
	match self.raw {
	    RawValue::Float(_) => true,
	    _ => false,
	}
    }

    pub fn new_boolean(value: bool) -> Self {
	Value {
	    raw: RawValue::Boolean(value),
	}
    }
    pub fn get_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
	match self.raw {
	    RawValue::Boolean(b) => Ok(b),
	    _ => Err(Box::new(Exception::new(Vec::new(), "not a boolean".to_string()))),
	}
    }
    pub fn is_boolean(&self) -> bool {
	match self.raw {
	    RawValue::Boolean(_) => true,
	    _ => false,
	}
    }

    pub fn new_symbol(value: Vec<String>) -> Self {
	Value {
	    raw: RawValue::Symbol(value),
	}
    }

    pub fn new_sexpr(value: Sexpr, context: &mut Context) -> Self {
	let gc_object = Gc::new(GcValue::Sexpr(value), context);
	context.send_gc(gc_object.clone());
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }

    pub fn new_function(value: Function, context: &mut Context) -> Self {
	let gc_object = Gc::new(GcValue::Function(value), context);
	context.send_gc(gc_object.clone());
	
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }

    pub fn get_function(&self) -> Result<&Function, Box<dyn std::error::Error>> {
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::Function(ref f) => Ok(f),
		    _ => Err(Box::new(Exception::new(Vec::new(), "not a function".to_string()))),
		}
	    }
	    _ => Err(Box::new(Exception::new(Vec::new(), "not a function".to_string()))),
	}
    }

    pub fn new_list(value: Vec<Value>, context: &mut Context) -> Self {
	let gc_object = Gc::new(GcValue::List(value), context);
	context.send_gc(gc_object.clone());
	
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }

    pub fn new_nil() -> Self {
	Value {
	    raw: RawValue::Nil,
	}
    }
    pub fn is_nil(&self) -> bool {
	match self.raw {
	    RawValue::Nil => true,
	    _ => false,
	}
    }

    pub fn mark(&mut self) {
	match self.raw {
	    RawValue::Gc(ref mut gc) => {
		gc.mark();
		match gc.get_mut() {
		    GcValue::List(ref mut list) => {
			for v in list {
			    v.mark();
			}
		    },
		    GcValue::Function(ref mut f) => {
			match f {
			    Function::Tree(_, _, frame, _) => {
				frame.mark();
			    }
			    Function::Native(_, _) => {},
			}
		    },
		    _ => {},
		}
		gc.mark();
	    },
	    _ => {},
	}
    }

    pub fn unmark(&mut self) {
	match self.raw {
	    RawValue::Gc(ref mut gc) => {
		gc.unmark();
		match gc.get_mut() {
		    GcValue::List(ref mut list) => {
			for v in list {
			    v.unmark();
			}
		    },
		    GcValue::Function(ref mut f) => {
			match f {
			    Function::Tree(_, _, frame, _) => {
				frame.unmark();
			    }
			    Function::Native(_, _) => {},
			}
		    },
		    _ => {},
		}
	    },
	    _ => {},
	}
    }
    
}

#[derive(Clone)]
enum RawValue {
    Gc(Gc<GcValue>),
    Integer(Integer),
    Float(f64),
    Boolean(bool),
    Symbol(Vec<String>),
    Nil,
}

pub enum GcValue {
    String(String),
    Sexpr(Sexpr),
    Function(Function),
    List(Vec<Value>),
}

#[derive(Clone)]
pub enum Function {
    Tree(Vec<String>, Sexpr, ContextFrame, FunctionShape),
    Native(fn(&mut Context, Vec<Value>, HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>>, FunctionShape),
}

#[derive(Clone)]
pub struct FunctionShape {
    args: Vec<String>,
}

impl FunctionShape {
    pub fn new(args: Vec<String>) -> Self {
	FunctionShape {
	    args,
	}
    }

    pub fn check(&self, name: &Vec<String>, args: &Vec<Value>, keyword_args: &HashMap<String, Value>) -> Result<(), Box<dyn std::error::Error>> {
	if self.args.len() != args.len() + keyword_args.len() {
	    Err(Box::new(Exception::new(name.clone(), "wrong number of arguments".to_string())))?;
	}

	for (i, arg) in self.args.iter().enumerate() {
	    if i < args.len() {
		continue;
	    } else {
		if !keyword_args.contains_key(arg) {
		    Err(Box::new(Exception::new(name.clone(), "invalid keyword".to_string())))?;
		}
	    }
	}

	Ok(())
    }
}