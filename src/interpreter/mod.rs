use std::error::Error;



pub mod walk_through;

pub mod context;
pub mod value;
pub mod module;

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
