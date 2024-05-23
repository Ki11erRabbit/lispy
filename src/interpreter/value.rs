use rug::Integer;
use std::collections::HashMap;
use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;
use crate::parser::Atom;
use crate::parser::Sexpr;
use crate::interpreter::{self, HelperResult};

use crate::gc::Gc;

use super::bytecode::Bytecode;
use super::{context::{ContextFrame, Context}, Exception, InterpreterResult};


pub struct Nil;


#[derive(Clone)]
pub struct Value {
    raw: RawValue, 
}

impl Value {
    pub fn new_string(value: &str, context: &Context) -> Self {
	let gc_object = Gc::new(GcValue::String(value.to_string()));
	context.send_gc(gc_object.clone());
	let raw = RawValue::Gc(gc_object);
	Value {
	    raw,
	}
    }
    pub fn new_string_from_string(value: String, context: &Context) -> Self {
	let gc_object = Gc::new(GcValue::String(value));
	context.send_gc(gc_object.clone());
	let raw = RawValue::Gc(gc_object);
	Value {
	    raw,
	}
    }
    pub fn get_string(&self, context: &Context) -> HelperResult<&String> {
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::String(ref s) => Ok(s),
		    _ => {
			let empty: Vec<&str> = Vec::new();
			Err(Box::new(Exception::new(&empty, "not a string", context)))
		    },
		}
	    },
	    _ => {
		let empty: Vec<&str> = Vec::new();
		Err(Box::new(Exception::new(&empty, "not a string", context)))
	    },
	}
    }
    pub fn get_string_mut(&mut self, context: &Context) -> HelperResult<&mut String> {
	match self.raw {
	    RawValue::Gc(ref mut gc) => {
		match gc.get_mut() {
		    GcValue::String(ref mut s) => Ok(s),
		    _ => {
			let empty: Vec<&str> = Vec::new();
			Err(Box::new(Exception::new(&empty, "not a string", context)))
		    },
		}
	    },
	    _ => {
		let empty: Vec<&str> = Vec::new();
		Err(Box::new(Exception::new(&empty, "not a string", context)))
	    },
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
    pub fn new_integer_from_usize(value: usize) -> Self {
	Value {
	    raw: RawValue::Integer(Integer::from(value)),
	}
    }
    pub fn new_integer_from_integer(value: Integer) -> Self {
	Value {
	    raw: RawValue::Integer(value),
	}
    }
    pub fn get_integer(&self, context: &Context) -> HelperResult<&Integer> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Integer(ref i) => Ok(i),
	    _ => Err(Box::new(Exception::new(&empty, "not an integer", context))),
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
    pub fn get_float(&self, context: &Context) -> HelperResult<f64> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Float(f) => Ok(f),
	    _ => Err(Box::new(Exception::new(&empty, "not a float", context))),
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
    pub fn get_boolean(&self, context: &Context) -> HelperResult<bool> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Boolean(b) => Ok(b),
	    _ => Err(Box::new(Exception::new(&empty, "not a boolean", context))),
	}
    }
    pub fn is_boolean(&self) -> bool {
	match self.raw {
	    RawValue::Boolean(_) => true,
	    _ => false,
	}
    }

    pub fn new_symbol(value: Vec<String>, context: &Context) -> Self {
	let gc_object = Gc::new(GcValue::Symbol(value));
	context.send_gc(gc_object.clone());
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }
    pub fn get_symbol(&self, context: &mut Context) -> HelperResult<&Vec<String>> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::Symbol(ref s) => Ok(s),
		    _ => Err(Box::new(Exception::new(&empty, "not a symbol", context))),
		}
	    },
	    _ => Err(Box::new(Exception::new(&empty, "not a symbol", context))),
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

    pub fn new_sexpr(value: Sexpr, context: &Context) -> Self {
	let gc_object = Gc::new(GcValue::Sexpr(value));
	context.send_gc(gc_object.clone());
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }

    pub fn new_function(value: Function, context: &Context) -> Self {
	let gc_object = Gc::new(GcValue::Function(value));
	context.send_gc(gc_object.clone());
	
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }
    pub fn get_function(&self, context: &Context) -> HelperResult<&Function> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::Function(ref f) => Ok(f),
		    _ => Err(Box::new(Exception::new(&empty, "not a function", context))),
		}
	    }
	    _ => Err(Box::new(Exception::new(&empty, "not a function", context))),
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

    pub fn new_vector(value: Vec<Value>, context: &Context) -> Self {
	let gc_object = Gc::new(GcValue::Vector(value));
	context.send_gc(gc_object.clone());
	
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }
    pub fn get_vector(&self, context: &Context) -> HelperResult<&Vec<Value>> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::Vector(ref v) => Ok(v),
		    _ => Err(Box::new(Exception::new(&empty, "not a vector", context))),
		}
	    }
	    _ => Err(Box::new(Exception::new(&empty, "not a vector", context))),
	}
    }
    pub fn get_vector_mut(&mut self, context: &Context) -> HelperResult<&mut Vec<Value>> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Gc(ref mut gc) => {
		match gc.get_mut() {
		    GcValue::Vector(ref mut v) => Ok(v),
		    _ => Err(Box::new(Exception::new(&empty, "not a vector", context))),
		}
	    }
	    _ => Err(Box::new(Exception::new(&empty, "not a vector", context))),
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

    pub fn new_pair(car: Value, cdr: Value, context: &Context) -> Self {
	let gc_object = Gc::new(GcValue::Pair((Box::new(car), Box::new(cdr))));
	context.send_gc(gc_object.clone());
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }
    pub fn get_pair(&self, context: &Context) -> HelperResult<(&Value, &Value)> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::Pair((ref car, ref cdr)) => Ok((car, cdr)),
		    _ => Err(Box::new(Exception::new(&empty, "not a pair", context))),
		}
	    },
	    _ => Err(Box::new(Exception::new(&empty, "not a pair", context))),
	}
    }
    pub fn get_pair_mut(&mut self, context: &Context) -> HelperResult<(&mut Value, &mut Value)> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Gc(ref mut gc) => {
		match gc.get_mut() {
		    GcValue::Pair((ref mut car, ref mut cdr)) => Ok((car, cdr)),
		    _ => Err(Box::new(Exception::new(&empty, "not a pair", context))),
		}
	    },
	    _ => Err(Box::new(Exception::new(&empty, "not a pair", context))),
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
    pub fn get_char(&self, context: &Context) -> HelperResult<char> {
	match self.raw {
	    RawValue::Char(c) => Ok(c),
	    _ => {
		let empty: Vec<&str> = Vec::new();
		Err(Box::new(Exception::new(&empty, "not a char", context)))
	    },
	}
    }
    pub fn is_char(&self) -> bool {
	match self.raw {
	    RawValue::Char(_) => true,
	    _ => false,
	}
    }

    pub fn new_rust_value(r: Box<dyn Any>, context: &Context) -> Self {
	let gc_object = Gc::new(GcValue::RustValue(r));
	context.send_gc(gc_object.clone());
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }
    pub fn is_rust_value(&self) -> bool {
	match self.raw {
	    RawValue::Gc(ref r) => {
		match r.get() {
		    GcValue::RustValue(_) => true,
		    _ => false,
		}
	    },
	    _ => false,
	}
    }
    pub fn get_rust_value(&self, context: &Context) -> HelperResult<&Box<dyn Any>> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::RustValue(r) => Ok(r),
		    _ => Err(Box::new(Exception::new(&empty, "not a rust value", context))),
		}
	    }
	    _ => Err(Box::new(Exception::new(&empty, "not a rust value", context))),
	}
    }
    pub fn get_rust_value_mut(&mut self, context: &Context) -> HelperResult<&mut Box<dyn Any>> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Gc(ref mut gc) => {
		match gc.get_mut() {
		    GcValue::RustValue(r) => Ok(r),
		    _ => Err(Box::new(Exception::new(&empty, "not a rust value", context))),
		}
	    }
	    _ => Err(Box::new(Exception::new(&empty, "not a rust value", context))),
	}
    }

    pub fn new_struct(s: Struct, context: &Context) -> Self {
	let gc_object = Gc::new(GcValue::Struct(s));
	context.send_gc(gc_object.clone());
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }
    pub fn is_struct(&self) -> bool {
	match self.raw {
	    RawValue::Gc(ref r) => {
		match r.get() {
		    GcValue::Struct(_) => true,
		    _ => false,
		}
	    },
	    _ => false,
	}
    }
    pub fn get_struct(&self, context: &Context) -> HelperResult<&Struct> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::Struct(ref s) => Ok(s),
		    _ => Err(Box::new(Exception::new(&empty, "not a struct", context))),
		}
	    }
	    _ => Err(Box::new(Exception::new(&empty, "not a struct", context))),
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

    pub fn mark(&self) {
	match self.raw {
	    RawValue::Gc(ref gc) => {
		gc.mark();
		match gc.get() {
		    GcValue::Vector(ref list) => {
			for v in list {
			    v.mark();
			}
		    },
		    GcValue::Function(ref f) => {
			match f {
			    Function::Tree(_, _, frame, _) => {
				frame.mark();
			    }
			    _ => {}
			}
		    },
		    GcValue::Pair((ref car, ref cdr)) => {
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

    pub fn unmark(&self) {
	match self.raw {
	    RawValue::Gc(ref gc) => {
		gc.unmark();
		match gc.get() {
		    GcValue::Vector(ref list) => {
			for v in list {
			    v.unmark();
			}
		    },
		    GcValue::Function(ref f) => {
			match f {
			    Function::Tree(_, _, frame, _) => {
				frame.unmark();
			    }
			    _ => {},
			}
		    },
		    GcValue::Pair((ref car, ref cdr)) => {
			car.unmark();
			cdr.unmark();
		    },
		    _ => {},
		}
	    },
	    _ => {},
	}
    }

    pub fn protect(&self) {
	match &self.raw {
	    RawValue::Gc(gc) => {
		gc.protect()
	    }
	    _ => {}
	}
    }

    pub fn get_type_index(&self) -> usize {
	match &self.raw {
	    RawValue::Gc(gc) => {
		match gc.get() {
		    GcValue::String(_) => 1,
		    GcValue::Sexpr(_) => 10,
		    GcValue::Function(_) => 8,
		    GcValue::Pair(_) => 6,
		    GcValue::Vector(_) => 7,
		    GcValue::Symbol(_) => 5,
		    GcValue::RustValue(_) => 11,
		    GcValue::Struct(s) => s.name_index ,
		}
	    },
	    RawValue::Integer(_) => 2,
	    RawValue::Float(_) => 3,
	    RawValue::Boolean(_) => 4,
	    RawValue::Nil => 0,
	    RawValue::Char(_) => 9,
	}
    }
    
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	match &self.raw {
	    RawValue::Gc(gc) => {
		write!(f, "{}", gc.get())
	    },
	    RawValue::Integer(i) => {
		write!(f, "{}", i)
	    },
	    RawValue::Float(fl) => {
		write!(f, "{}", fl)
	    },
	    RawValue::Boolean(b) => {
		write!(f, "{}", b)
	    },
	    RawValue::Char(c) => {
		match c {
		    '\n' => write!(f, "#\\newline"),
		    '\t' => write!(f, "#\\tab"),
		    ' ' => write!(f, "#\\space"),
		    '\r' => write!(f, "#\\return"),
		    '\0' => write!(f, "#\\null"),
		    _ => write!(f, "#\\{}", c),
		}
	    },
	    RawValue::Nil => {
		write!(f, "nil")
	    },
	}
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	write!(f, "{}", self)
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
    RustValue(Box<dyn Any>),
    Struct(Struct),
}

impl std::fmt::Display for GcValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	match self {
	    GcValue::String(s) => {
		write!(f, "{}", s)
	    },
	    GcValue::Sexpr(s) => {
		write!(f, "{:?}", s)
	    },
	    GcValue::Function(_) => {
		write!(f, "<procedure>")
	    },
	    GcValue::Pair((car, cdr)) => {
		write!(f, "'(")?;
		let mut current = Some(car.as_ref());
		while let Some(car) = current {
		    write!(f, "{}", car)?;
		    if cdr.is_pair() {
			current = Some(cdr.get_pair(&mut Context::default()).expect("cdr is not a pair").0);
		    } else {
			write!(f, " {}", cdr)?;
		    }
		}
		write!(f, ")")
	    },
	    GcValue::Vector(v) => {
		write!(f, "#({})", v.iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join(" "))
	    },
	    GcValue::Symbol(s) => {
		write!(f, "'{}", s.join("."))
	    },
	    GcValue::RustValue(_) => {
		write!(f, "<rust value>")
	    },
	    GcValue::Struct(_) => {
		write!(f, "<struct>")
	    },
	}
    }
}

impl std::fmt::Debug for GcValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	write!(f, "{}", self)
    }
}

#[derive(Clone)]
pub enum Function {
    Tree(Vec<String>, Sexpr, ContextFrame, FunctionShape),
    Native(fn(&mut Context, Vec<Value>, HashMap<String, Value>) -> HelperResult<Value>, FunctionShape),
    Bytecode(Vec<String>, Vec<Bytecode>, FunctionShape),
    //NativeClosure(Box<dyn Fn(&mut Context, Vec<Value>, HashMap<String, Value>) -> HelperResult<Value>>, FunctionShape),
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
    
    pub fn call(&self, name: &Vec<String>, list: &[Sexpr], context: &mut Context) -> InterpreterResult {
	match self {
	    Function::Tree(fun_args, body, frame, shape) => {
		let mut args = Vec::new();
		let mut keyword_args = std::collections::HashMap::new();
		let mut iterator = list.iter();
		while let Some(sexpr) = iterator.next() {
		    match sexpr {
			Sexpr::Atom(Atom::Keyword(k)) => {
			    if let Some(value) = iterator.next() {
				match interpreter::walk_through::walk_through(value, context)? {
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
			    match interpreter::walk_through::walk_through(s, context)? {
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
        
		let value = interpreter::walk_through::walk_through(&body, context);
		context.pop_frame();
		value
	    },
	    Function::Native(f, shape) => {
		let mut args = Vec::new();
		let mut keyword_args = std::collections::HashMap::new();
		let mut iterator = list.iter();
		while let Some(sexpr) = iterator.next() {
		    match sexpr {
			Sexpr::Atom(Atom::Keyword(k)) => {
			    if let Some(value) = iterator.next() {
				match interpreter::walk_through::walk_through(value, context)? {
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
			    match interpreter::walk_through::walk_through(s, context)? {
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
		let mut keyword_args = std::collections::HashMap::new();
		let mut iterator = list.iter();
		while let Some(sexpr) = iterator.next() {
		    match sexpr {
			Sexpr::Atom(Atom::Keyword(k)) => {
			    if let Some(value) = iterator.next() {
				match interpreter::walk_through::walk_through(value, context)? {
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
			    match interpreter::walk_through::walk_through(s, context)? {
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
		
        
		let value = interpreter::bytecode::run(&bytecode.as_slice(), context);
		context.pop_frame();
		value
	    },
	}
    }

    pub fn call_from_bytecode(&self, name: &Vec<String>, args: Vec<Value>, kargs: HashMap<String, Value>, context: &mut Context) -> InterpreterResult {
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
        
		let value = interpreter::walk_through::walk_through(&body, context);
		context.pop_frame();
		value
	    },
	    Function::Native(f, shape) => {
		shape.check(&name, &args, &kargs, context)?;
		Ok(Some(f(context, args, kargs)?))
	    }
	    Function::Bytecode(fun_args, bytecode, shape) => {
		context.push_frame(None);

		for (arg, value) in fun_args.iter().zip(args.iter()) {
		    context.define(arg, value.clone());
		}
		for (arg, value) in kargs.iter() {
		    context.define(arg, value.clone());
		}

		shape.check(&name, &args, &kargs, context)?;

		let value = interpreter::bytecode::run(&bytecode.as_slice(), context);
		context.pop_frame();
		value
	    },
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

    pub fn check(&self, name: &Vec<String>, args: &Vec<Value>, keyword_args: &HashMap<String, Value>, context: &mut Context) -> HelperResult<()> {
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
}

pub struct Struct {
    name_index: usize,
    members: Box<[Value]>,
}

impl Struct {
    pub fn new(name_index: usize, members: Box<[Value]>) -> Self {
	Struct {
	    name_index,
	    members,
	}
    }

    pub fn get_member(&self, index: usize, context: &Context) -> HelperResult<&Value> {
	let empty: Vec<&str> = Vec::new();
	if index >= self.members.len() {
	    Err(Box::new(Exception::new(&empty, "index out of bounds", context)))
	} else {
	    Ok(&self.members[index])
	}
    }
    /*pub fn create_functions(name: Vec<String>, member_names: Vec<Vec<String>>, context: &mut Context) {
	let mut functions = HashMap::new();
	let constructor_shape = FunctionShape::new(member_names.iter().map(|v| v.join(".")).collect());
	let constructor = |context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>| -> HelperResult<Value> {
	    let mut index = 0;
	    let mut members = Vec::new();
	    for arg in args {
		index += 1;
		members.push(arg);
	    }
	    for member in member_names.iter().skip(index) {
		if let Some(value) = keyword_args.get(&member.join(".")) {
		    members.push(value.clone());
		} else {
		    let empty: Vec<&str> = Vec::new();
		    return Err(Box::new(Exception::new(&empty, "missing keyword argument", context)));
		}
	    }
	    Ok(Value::new_struct(Struct {
		name_index: context.get_or_create_type_symbol(&name.clone()),
		members: members.into_boxed_slice(),
	    }, context))
	};
	functions.insert(name, Value::new_function(Function::NativeClosure(Box::new(constructor), constructor_shape), context));
	
	let mut accessor_shapes = Vec::new();
	let mut member_names = member_names.iter().map(|m| (*m).clone()).collect::<Vec<Vec<String>>>();
	for member in member_names.iter_mut() {
	    let new_end = name.last().cloned().unwrap() + "-" + member.last_mut().unwrap();
	    *member.last_mut().unwrap() = new_end;
	    accessor_shapes.push(FunctionShape::new(member.clone()));
	}

	for (i, member) in member_names.iter().enumerate() {
	    let accessor = |context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>| -> HelperResult<Value> {
		let mut index = 0;
		let mut members = Vec::new();
		for arg in args {
		    index += 1;
		    members.push(arg);
		}
		for member in member_names.iter().skip(index) {
		    if let Some(value) = keyword_args.get(&member.join(".")) {
			members.push(value.clone());
		    } else {
			let empty: Vec<&str> = Vec::new();
			return Err(Box::new(Exception::new(&empty, "missing keyword argument", context)));
		    }
		}
		Ok(members[i].clone())
	    };
	    functions.insert((*member).clone(), Value::new_function(Function::NativeClosure(Box::new(accessor), accessor_shapes[i].clone()), context));
	}

	for (name, function) in functions {
	    context.define(&name[0], function);
	}
    }*/

}

pub struct Enum {
    name_index: usize,
    variant_index: usize,
    members: Struct,
}
