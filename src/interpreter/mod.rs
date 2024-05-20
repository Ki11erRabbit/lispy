use std::error::Error;
use crate::interpreter::value::Value;


pub mod walk_through;

pub mod context;
pub mod value;
pub mod module;


pub type InterpreterResult = Result<Option<Value>, Box<Exception>>;
pub type HelperResult<T> = std::result::Result<T, Box<Exception>>;

#[derive(Debug)]
pub struct Exception {
    who: Vec<String>,
    message: String,
}

impl Exception {
    pub fn new(who: Vec<String>, message: String) -> Self {
	Exception {
	    who,
	    message,
	}
    }

    pub fn get_who(&self) -> &Vec<String> {
	&self.who
    }
}

impl Error for Exception {}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	if self.who.len() == 0 {
	    return write!(f, "Error: {}", self.message);
	}
	let who = self.who.join(".");
	write!(f, "Error: {} {}", who, self.message)
    }
}
