use rug::Integer;
use std::collections::HashMap;
use crate::parser::Sexpr;

use super::context::ContextFrame;





#[derive(Clone)]
pub struct Value {
    pub raw: RawValue,
}

impl Value {
    pub fn new_string(value: &str) -> Self {
	Value {
	    raw: RawValue::String(value.to_string()),
	}
    }
    pub fn get_string(&self) -> Result<&String, Box<dyn std::error::Error>> {
	match &self.raw {
	    RawValue::String(s) => Ok(s),
	    _ => todo!("error"),
	}
    }
    pub fn is_string(&self) -> bool {
	match &self.raw {
	    RawValue::String(_) => true,
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
	match &self.raw {
	    RawValue::Integer(i) => Ok(i),
	    _ => todo!("error"),
	}
    }

    pub fn new_float(value: f64) -> Self {
	Value {
	    raw: RawValue::Float(value),
	}
    }
    pub fn get_float(&self) -> Result<f64, Box<dyn std::error::Error>> {
	match &self.raw {
	    RawValue::Float(f) => Ok(*f),
	    _ => todo!("error"),
	}
    }
    pub fn is_float(&self) -> bool {
	match &self.raw {
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
	match &self.raw {
	    RawValue::Boolean(b) => Ok(*b),
	    _ => todo!("error"),
	}
    }

    pub fn new_symbol(value: Vec<String>) -> Self {
	Value {
	    raw: RawValue::Symbol(value),
	}
    }

    pub fn new_sexpr(value: Sexpr) -> Self {
	Value {
	    raw: RawValue::Sexpr(value),
	}
    }

    pub fn new_function(value: Function) -> Self {
	Value {
	    raw: RawValue::Function(value),
	}
    }

    pub fn get_function(&self) -> Result<&Function, Box<dyn std::error::Error>> {
	match &self.raw {
	    RawValue::Function(f) => Ok(f),
	    _ => todo!("error"),
	}
    }

    pub fn new_list(value: Vec<Value>) -> Self {
	Value {
	    raw: RawValue::List(value),
	}
    }

    pub fn new_nil() -> Self {
	Value {
	    raw: RawValue::Nil,
	}
    }
    pub fn is_nil(&self) -> bool {
	match &self.raw {
	    RawValue::Nil => true,
	    _ => false,
	}
    }
    
}

#[derive(Clone)]
enum RawValue {
    String(String),
    Integer(Integer),
    Float(f64),
    Boolean(bool),
    Symbol(Vec<String>),
    Sexpr(Sexpr),
    Function(Function),
    List(Vec<Value>),
    Nil,
}

#[derive(Clone)]
pub enum Function {
    Tree(Vec<String>, Sexpr, ContextFrame, FunctionShape),
    Native(fn(Vec<Value>, HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>>, FunctionShape),
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

    pub fn check(&self, args: &Vec<Value>, keyword_args: &HashMap<String, Value>) -> Result<(), Box<dyn std::error::Error>> {
	if self.args.len() != args.len() + keyword_args.len() {
	    todo!("error");
	}

	for (i, arg) in self.args.iter().enumerate() {
	    if i < args.len() {
		continue;
	    } else {
		if !keyword_args.contains_key(arg) {
		    todo!("error");
		}
	    }
	}

	Ok(())
    }
}
