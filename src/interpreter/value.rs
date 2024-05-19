use rug::Integer;

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
    
    pub fn new_integer(value: &str) -> Self {
	Value {
	    raw: RawValue::Integer(Integer::from_str_radix(value, 10).unwrap()),
	}
    }

    pub fn new_float(value: f64) -> Self {
	Value {
	    raw: RawValue::Float(value),
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
}

#[derive(Clone)]
pub enum Function {
    Tree(Vec<String>, Sexpr, ContextFrame),

}
