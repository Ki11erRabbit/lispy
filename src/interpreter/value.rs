use rug::Integer;
use std::collections::HashMap;
use crate::parser::Sexpr;
use crate::interpreter::HelperResult;

use crate::gc::Gc;

use super::{context::{ContextFrame, Context}, Exception};


pub struct Nil;


#[derive(Clone)]
pub struct Value {
    raw: RawValue, 
}

impl Value {
    pub fn new_string(value: &str, context: &mut Context) -> Self {
	let gc_object = Gc::new(GcValue::String(value.to_string()));
	context.send_gc(gc_object.clone());
	let raw = RawValue::Gc(gc_object);
	Value {
	    raw,
	}
    }
    pub fn get_string(&self) -> HelperResult<&String> {
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
    pub fn get_integer(&self) -> HelperResult<&Integer> {
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
    pub fn get_float(&self) -> HelperResult<f64> {
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
    pub fn get_boolean(&self) -> HelperResult<bool> {
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

    pub fn new_symbol(value: Vec<String>, context: &mut Context) -> Self {
	let gc_object = Gc::new(GcValue::Symbol(value));
	context.send_gc(gc_object.clone());
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }
    pub fn is_symbol(&self) -> bool {
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::Symbol(_) => true,
		    _ => false,
		}
	    },
	    _ => false,
	}
    }

    pub fn new_sexpr(value: Sexpr, context: &mut Context) -> Self {
	let gc_object = Gc::new(GcValue::Sexpr(value));
	context.send_gc(gc_object.clone());
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }

    pub fn new_function(value: Function, context: &mut Context) -> Self {
	let gc_object = Gc::new(GcValue::Function(value));
	context.send_gc(gc_object.clone());
	
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }
    pub fn get_function(&self) -> HelperResult<&Function> {
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
    pub fn is_function(&self) -> bool {
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::Function(_) => true,
		    _ => false,
		}
	    }
	    _ => false,
	}
    }

    pub fn new_vector(value: Vec<Value>, context: &mut Context) -> Self {
	let gc_object = Gc::new(GcValue::Vector(value));
	context.send_gc(gc_object.clone());
	
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }
    pub fn get_vector(&self) -> HelperResult<&Vec<Value>> {
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::Vector(ref v) => Ok(v),
		    _ => Err(Box::new(Exception::new(Vec::new(), "not a vector".to_string()))),
		}
	    }
	    _ => Err(Box::new(Exception::new(Vec::new(), "not a vector".to_string()))),
	}
    }
    pub fn is_vector(&self) -> bool {
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::Vector(_) => true,
		    _ => false,
		}
	    }
	    _ => false,
	}
    }

    pub fn new_pair(car: Value, cdr: Value, context: &mut Context) -> Self {
	let gc_object = Gc::new(GcValue::Pair((Box::new(car), Box::new(cdr))));
	context.send_gc(gc_object.clone());
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }
    pub fn get_pair(&self) -> HelperResult<(&Value, &Value)> {
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::Pair((ref car, ref cdr)) => Ok((car, cdr)),
		    _ => Err(Box::new(Exception::new(Vec::new(), "not a pair".to_string()))),
		}
	    },
	    _ => Err(Box::new(Exception::new(Vec::new(), "not a pair".to_string()))),
	}
    }
    pub fn is_pair(&self) -> bool {
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::Pair(_) => true,
		    _ => false,
		}
	    },
	    _ => false,
	}
    }
    pub fn new_char(c: char) -> Self {
	Value {
	    raw: RawValue::Char(c),
	}
    }
    pub fn get_char(&self) -> HelperResult<char> {
	match self.raw {
	    RawValue::Char(c) => Ok(c),
	    _ => Err(Box::new(Exception::new(Vec::new(), "not a char".to_string()))),
	}
    }
    pub fn is_char(&self) -> bool {
	match self.raw {
	    RawValue::Char(_) => true,
	    _ => false,
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
		    GcValue::Vector(ref mut list) => {
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
		    GcValue::Pair((ref mut car, ref mut cdr)) => {
			car.mark();
			cdr.mark();
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
		    GcValue::Vector(ref mut list) => {
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
		    GcValue::Pair((ref mut car, ref mut cdr)) => {
			car.unmark();
			cdr.unmark();
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
    Nil,
    Char(char),
}

pub enum GcValue {
    String(String),
    Sexpr(Sexpr),
    Function(Function),
    Pair((Box<Value>, Box<Value>)),
    Vector(Vec<Value>),
    Symbol(Vec<String>),
}

#[derive(Clone)]
pub enum Function {
    Tree(Vec<String>, Sexpr, ContextFrame, FunctionShape),
    Native(fn(&mut Context, Vec<Value>, HashMap<String, Value>) -> HelperResult<Value>, FunctionShape),
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

    pub fn check(&self, name: &Vec<String>, args: &Vec<Value>, keyword_args: &HashMap<String, Value>) -> HelperResult<()> {
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
