use std::error::Error;
use crate::interpreter::value::Value;

use self::context::Context;


pub mod walk_through;

pub mod context;
pub mod value;
pub mod module;

pub type InterpreterResult = Result<Option<Value>, Box<Exception>>;
pub type HelperResult<T> = std::result::Result<T, Box<Exception>>;

#[derive(Debug)]
pub struct Exception {
    who: Value, // Symbol
    message: Value, // String
}

impl Exception {
    pub fn new<S: AsRef<str>>(who: &Vec<S>, message: &str, context: &Context) -> Self {
	let who = who.iter().map(|s| s.as_ref().to_string()).collect();
	let who = Value::new_symbol(who, context);
	let message = Value::new_string(message, context);
	Exception {
	    who,
	    message,
	}
    }

    pub fn get_who(&self, context: &mut Context) -> &Vec<String> {
	self.who.get_symbol(context).expect("who is not a symbol")
    }
}

impl Error for Exception {}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	write!(f, "{}: {}", self.who, self.message)
    }
}
