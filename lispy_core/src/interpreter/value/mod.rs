use std::{any::Any, ffi::{CString, c_char, c_void}};
use rug::Integer;
use crate::parser::Sexpr;
use crate::interpreter::HelperResult;
use crate::interpreter::value::r#struct::Struct;
use crate::interpreter::value::r#enum::Enum;
use crate::interpreter::value::function::{Function, FunctionShape, CFunctionOutput};
use crate::interpreter::kwargs::Kwargs;


use crate::gc::Gc;

use super::{context::Context, Exception};

pub mod function;
pub mod r#struct;
pub mod r#enum;

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
    pub fn new_string_from_c(value: *mut c_char, len: usize, context: &Context) -> Self {
	let pointer: *mut i8;
	unsafe {
	    pointer = std::alloc::alloc(std::alloc::Layout::array::<c_char>(len).unwrap()) as *mut i8;
	    std::ptr::copy(pointer, value, 0);
	}
	let value = unsafe { CString::from_raw(pointer) };
	let value = value.to_str().unwrap();
	let gc_object = Gc::new(GcValue::String(value.to_string()));
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
    pub fn new_symbol_from_c(value: *mut *mut c_char, len: usize, str_lens: *mut usize, context: &Context) -> Self {
	let mut value = value;
	let mut str_lens = str_lens;
	let mut symbols = Vec::new();
	for _ in 0..len {
	    let symbol = unsafe { *value };
	    let len = unsafe { *str_lens };
	    let pointer: *mut i8;
	    unsafe {
		pointer = std::alloc::alloc(std::alloc::Layout::array::<c_char>(len).unwrap()) as *mut i8;
		std::ptr::copy(pointer, symbol, 0);
	    }
	    let symbol = unsafe { CString::from_raw(pointer) };
	    let symbol = symbol.to_str().unwrap();
	    symbols.push(symbol.to_string());
	    unsafe {
		value = value.add(1);
		str_lens = str_lens.add(1);
	    }
	}
	let gc_object = Gc::new(GcValue::Symbol(symbols));
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
    pub fn get_struct_mut(&mut self, context: &Context) -> HelperResult<&mut Struct> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Gc(ref mut gc) => {
		match gc.get_mut() {
		    GcValue::Struct(ref mut s) => Ok(s),
		    _ => Err(Box::new(Exception::new(&empty, "not a struct", context))),
		}
	    }
	    _ => Err(Box::new(Exception::new(&empty, "not a struct", context))),
	}
    }
    pub fn new_enum(e: Enum, context: &Context) -> Self {
	let gc_object = Gc::new(GcValue::Enum(e));
	context.send_gc(gc_object.clone());
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }
    pub fn is_enum(&self) -> bool {
	match self.raw {
	    RawValue::Gc(ref r) => {
		match r.get() {
		    GcValue::Enum(_) => true,
		    _ => false,
		}
	    },
	    _ => false,
	}
    }
    pub fn get_enum(&self, context: &Context) -> HelperResult<&Enum> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::Enum(ref e) => Ok(e),
		    _ => Err(Box::new(Exception::new(&empty, "not an enum", context))),
		}
	    }
	    _ => Err(Box::new(Exception::new(&empty, "not an enum", context))),
	}
    }
    pub fn get_enum_mut(&mut self, context: &Context) -> HelperResult<&mut Enum> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Gc(ref mut gc) => {
		match gc.get_mut() {
		    GcValue::Enum(ref mut e) => Ok(e),
		    _ => Err(Box::new(Exception::new(&empty, "not an enum", context))),
		}
	    }
	    _ => Err(Box::new(Exception::new(&empty, "not an enum", context))),
	}
    }

    pub fn new_bytevector(value: Vec<u8>, context: &Context) -> Self {
	let gc_object = Gc::new(GcValue::ByteVector(value));
	context.send_gc(gc_object.clone());
	Value {
	    raw: RawValue::Gc(gc_object),
	}
    }
    pub fn get_bytevector(&self, context: &Context) -> HelperResult<&Vec<u8>> {
	let empty: Vec<&str> = Vec::new();
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::ByteVector(ref v) => Ok(v),
		    _ => Err(Box::new(Exception::new(&empty, "not a bytevector", context))),
		}
	    }
	    _ => Err(Box::new(Exception::new(&empty, "not a bytevector", context))),
	}
    }
    pub fn is_bytevector(&self) -> bool {
	match self.raw {
	    RawValue::Gc(ref gc) => {
		match gc.get() {
		    GcValue::ByteVector(_) => true,
		    _ => false,
		}
	    }
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
		    GcValue::Struct(s) => s.get_name_index(),
		    GcValue::Enum(e) => e.get_name_index(),
		    GcValue::ByteVector(_) => 12,
		    GcValue::CValue(_, _) => 13,
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

// C API Functions for Value
impl Value {
    #[no_mangle]
    pub extern "C" fn value_new_nil() -> *mut Self {
	Box::into_raw(Box::new(Value::new_nil()))
    }

    #[no_mangle]
    pub extern "C" fn value_new_string(value: *mut c_char, len: usize, context: *mut Context) -> *mut Self {
	let context = unsafe { &mut *context };
	let value = Value::new_string_from_c(value, len, context);
	Box::into_raw(Box::new(value))
    }

    #[no_mangle]
    pub extern "C" fn value_new_integer(value: *mut c_char, len: usize) -> *mut Self {
	let pointer: *mut i8;
	unsafe {
	    pointer = std::alloc::alloc(std::alloc::Layout::array::<c_char>(len).unwrap()) as *mut i8;
	    std::ptr::copy(pointer, value, 0);
	}
	let value = unsafe { CString::from_raw(pointer) };
	let value = value.to_str().unwrap();
	Box::into_raw(Box::new(Value::new_integer(value)))
    }

    #[no_mangle]
    pub extern "C" fn value_new_integer_from_ssize_t(value: isize) -> *mut Self {
	Box::into_raw(Box::new(Value::new_integer_from_integer(Integer::from(value))))
    }
 
    #[no_mangle]
    pub extern "C" fn value_new_float(value: f64) -> *mut Self {
	Box::into_raw(Box::new(Value::new_float(value)))
    }

    #[no_mangle]
    pub extern "C" fn value_new_boolean(value: bool) -> *mut Self {
	Box::into_raw(Box::new(Value::new_boolean(value)))
    }

    #[no_mangle]
    pub extern "C" fn value_new_symbol(value: *mut *mut c_char, len: usize, str_lens: *mut usize, context: *mut Context) -> *mut Self {
	let context = unsafe { &mut *context };
	let value = Value::new_symbol_from_c(value, len, str_lens, context);
	Box::into_raw(Box::new(value))
    }

    #[no_mangle]
    pub extern "C" fn value_new_char(value: u32) -> *mut Self {
	Box::into_raw(Box::new(Value::new_char(unsafe {char::from_u32_unchecked(value)})))
    }

    #[no_mangle]
    pub extern "C" fn value_new_pair(car: *mut Self, cdr: *mut Self, context: *mut Context) -> *mut Self {
	let context = unsafe { &mut *context };
	let car = unsafe { Box::from_raw(car) };
	let cdr = unsafe { Box::from_raw(cdr) };
	Box::into_raw(Box::new(Value::new_pair(*car, *cdr, context)))
    }

    #[no_mangle]
    pub extern "C" fn value_new_vector(value: *mut *mut Self, len: usize, context: *mut Context) -> *mut Self {
	let context = unsafe { &mut *context };
	let mut value = value;
	let mut values = Vec::new();
	for _ in 0..len {
	    let v = unsafe { Box::from_raw(*value) };
	    values.push(*v);
	    unsafe {value = value.add(1);}
	}
	Box::into_raw(Box::new(Value::new_vector(values, context)))
    }

    #[no_mangle]
    pub extern "C" fn value_new_c_value(value: *mut c_void, free: unsafe extern "C" fn(*mut c_void), context: *mut Context) -> *mut Self {
	let context = unsafe { &mut *context };
	let gc_object = Gc::new(GcValue::CValue(value, free));
	context.send_gc(gc_object.clone());
	Box::into_raw(Box::new(Value {
	    raw: RawValue::Gc(gc_object),
	}))
    }

    #[no_mangle]
    pub extern "C" fn value_new_function(value: unsafe extern "C" fn(*mut Context, *mut *mut Value, usize, *mut Kwargs, *mut CFunctionOutput), shape: *mut FunctionShape, context: *mut Context) -> *mut Self {
	let context = unsafe { &mut *context };
	let shape = unsafe { Box::from_raw(shape) };
	let function = Function::CNative(value, *shape);
	let value = Value::new_function(function, context);
	Box::into_raw(Box::new(value))
    }

    #[no_mangle]
    pub extern "C" fn value_new_bytevector(value: *mut u8, len: usize, context: *mut Context) -> *mut Self {
	let context = unsafe { &mut *context };
	let mut value = value;
	let mut values = Vec::new();
	for _ in 0..len {
	    let v = unsafe { *value };
	    values.push(v);
	    unsafe {value = value.add(1);}
	}
	Box::into_raw(Box::new(Value::new_bytevector(values, context)))
    }

    #[no_mangle]
    pub extern "C" fn value_get_string(value: *mut Self, context: *mut Context) -> *const c_char {
	let value = unsafe { &*value };
	let context = unsafe { &mut *context };
	let result = value.get_string(context);
	match result {
	    Ok(s) => {
		let s = s.as_str();
		let c_string = CString::new(s).unwrap();
		c_string.into_raw()
	    },
	    Err(_) => {
		let c_string = CString::new("").unwrap();
		c_string.into_raw()
	    },
	}
    }

    #[no_mangle]
    pub extern "C" fn value_free_string(value: *mut c_char) {
	unsafe {
	    drop(CString::from_raw(value));
	}
    }


    #[no_mangle]
    pub extern "C" fn value_get_float(value: *mut Self, context: *mut Context) -> f64 {
	let value = unsafe { &*value };
	let context = unsafe { &mut *context };
	let result = value.get_float(context);
	match result {
	    Ok(f) => f,
	    Err(_) => 0.0,
	}
    }

    #[no_mangle]
    pub extern "C" fn value_get_boolean(value: *mut Self, context: *mut Context) -> bool {
	let value = unsafe { &*value };
	let context = unsafe { &mut *context };
	let result = value.get_boolean(context);
	match result {
	    Ok(b) => b,
	    Err(_) => false,
	}
    }

    #[no_mangle]
    pub extern "C" fn value_get_symbol(value: *mut Self, context: *mut Context) -> *mut *mut c_char {
	let value = unsafe { &*value };
	let context = unsafe { &mut *context };
	let result = value.get_symbol(context);
	match result {
	    Ok(s) => {
		let mut symbols = Vec::new();
		for symbol in s.iter() {
		    let c_string = CString::new(symbol.as_str()).unwrap();
		    symbols.push(c_string.into_raw());
		}
		let mut symbols = symbols.into_boxed_slice();
		let ptr = symbols.as_mut_ptr();
		std::mem::forget(symbols);
		ptr
	    },
	    Err(_) => {
		std::ptr::null_mut()
	    },
	}
    }

    #[no_mangle]
    pub extern "C" fn value_free_symbol(value: *mut *mut c_char) {
	let mut value = value;
	while !value.is_null() {
	    let symbol = unsafe { *value };
	    unsafe {
		drop(CString::from_raw(symbol));
		value = value.add(1);
	    }
	}
    }

    #[no_mangle]
    pub extern "C" fn value_get_char(value: *mut Self, context: *mut Context) -> u32 {
	let value = unsafe { &*value };
	let context = unsafe { &mut *context };
	let result = value.get_char(context);
	match result {
	    Ok(c) => c as u32,
	    Err(_) => 0,
	}
    }

    #[no_mangle]
    pub extern "C" fn value_get_pair(value: *mut Self, context: *mut Context) -> *mut *mut Self {
	let value = unsafe { &*value };
	let context = unsafe { &mut *context };
	let result = value.get_pair(context);
	match result {
	    Ok((car, cdr)) => {
		let car = Box::into_raw(Box::new(car.clone()));
		let cdr = Box::into_raw(Box::new(cdr.clone()));
		let mut pair = Vec::new();
		pair.push(car);
		pair.push(cdr);
		let mut pair = pair.into_boxed_slice();
		let ptr = pair.as_mut_ptr();
		std::mem::forget(pair);
		ptr
	    },
	    Err(_) => std::ptr::null_mut(),
	}
    }

    #[no_mangle]
    pub extern "C" fn value_free_pair(value: *mut *mut Self) {
	let mut value = value;
	while !value.is_null() {
	    let v = unsafe { *value };
	    unsafe {
		drop(Box::from_raw(v));
		value = value.add(1);
	    }
	}
    }

    #[no_mangle]
    pub extern "C" fn value_get_vector(value: *mut Self, context: *mut Context) -> *mut *mut Self {
	let value = unsafe { &*value };
	let context = unsafe { &mut *context };
	let result = value.get_vector(context);
	match result {
	    Ok(v) => {
		let mut values = Vec::new();
		for value in v.iter() {
		    values.push(Box::into_raw(Box::new(value.clone())));
		}
		let mut values = values.into_boxed_slice();
		let ptr = values.as_mut_ptr();
		std::mem::forget(values);
		ptr
	    },
	    Err(_) => {
		return std::ptr::null_mut();
	    },
	}
    }

    #[no_mangle]
    pub extern "C" fn value_free_vector(value: *mut *mut Self) {
	let mut value = value;
	while !value.is_null() {
	    let v = unsafe { *value };
	    unsafe {
		drop(Box::from_raw(v));
		value = value.add(1);
	    }
	}
    }

    #[no_mangle]
    pub extern "C" fn value_get_bytevector(value: *mut Self, context: *mut Context) -> *mut u8 {
	let value = unsafe { &*value };
	let context = unsafe { &mut *context };
	let result = value.get_bytevector(context);
	match result {
	    Ok(v) => {
		let mut values = Vec::new();
		for value in v.iter() {
		    values.push(*value);
		}
		let mut values = values.into_boxed_slice();
		let ptr = values.as_mut_ptr();
		std::mem::forget(values);
		ptr
	    },
	    Err(_) => {
		return std::ptr::null_mut();
	    },
	}
    }

    #[no_mangle]
    pub extern "C" fn value_free_bytevector(value: *mut u8) {
	unsafe {
	    drop(Box::from_raw(value));
	}
    }

    #[no_mangle]
    pub extern "C" fn value_get_c_value(value: *mut Self) -> *mut c_void {
	let value = unsafe { &*value };
	match &value.raw {
	    RawValue::Gc(gc) => {
		match gc.get() {
		    GcValue::CValue(v, _) => *v,
		    _ => std::ptr::null_mut(),
		}
	    },
	    _ => std::ptr::null_mut(),
	}
    }

    #[no_mangle]
    pub extern "C" fn value_get_integer_as_i64(value: *mut Self) -> i64 {
	let value = unsafe { &*value };
	match &value.raw {
	    RawValue::Integer(i) => i.to_i64().unwrap(),
	    _ => 0,
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
    Enum(Enum),
    ByteVector(Vec<u8>),
    CValue(*mut c_void, unsafe extern "C" fn(*mut c_void)),
}

impl Drop for GcValue {
    fn drop(&mut self) {
	match self {
	    GcValue::CValue(value, free) => {
		unsafe {
		    free(*value);
		}
	    },
	    _ => {},
	}
    }
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
	    GcValue::Enum(_) => {
		write!(f, "<enum>")
	    },
	    GcValue::ByteVector(v) => {
		write!(f, "#u8({})", v.iter().map(|b| format!("{}", b)).collect::<Vec<String>>().join(" "))
	    },
	    GcValue::CValue(_, _) => {
		write!(f, "<c value>")
	    },
	}
    }
}

impl std::fmt::Debug for GcValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	write!(f, "{}", self)
    }
}
