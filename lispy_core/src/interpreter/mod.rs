pub mod value;
pub mod module;
pub mod kwargs;
pub mod context;
pub mod walkthrough;
pub mod bytecode;

use std::{error::Error, ffi::c_char};
use crate::interpreter::value::Value;
use self::context::Context;

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
    pub fn get_message(&self) -> Value {
	self.message.clone()
    }

    #[no_mangle]
    pub extern "C" fn exception_new(who: *mut *mut c_char, symbol_len: usize, symbol_lens: *usize, message: *mut c_char, string_len: usize, context: *mut Context) -> *mut Exception {
	let who = Value::new_symbol_from_c(who, symbol_len, symbol_lens, context);
	let message = Value::new_string_from_c(message, string_len, context);
	let exception = Box::new(Exception {
	    who,
	    message,
	});
	Box::into_raw(exception)
    }
	
}

impl Error for Exception {}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	write!(f, "{}: {}", self.who, self.message)
    }
}
