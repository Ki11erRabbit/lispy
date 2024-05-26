use std::{collections::HashMap, ffi::c_char};
use crate::interpreter::value::Value;


#[repr(C)]
#[derive(Clone)]
pub struct Kwargs {
    keywords: HashMap<String, Value>,
}

impl Kwargs {
    pub fn new() -> Self {
	Kwargs {
	    keywords: HashMap::new(),
	}
    }

    pub fn insert(&mut self, key: String, value: Value) {
	self.keywords.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
	self.keywords.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
	self.keywords.get_mut(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
	self.keywords.remove(key)
    }

    pub fn clear(&mut self) {
	self.keywords.clear();
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Value> {
	self.keywords.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<String, Value> {
	self.keywords.iter_mut()
    }

    pub fn contains_key(&self, key: &str) -> bool {
	self.keywords.contains_key(key)
    }

    #[no_mangle]
    pub extern "C" fn get_value(&self, key: *const c_char) -> *const Value {
	let key = unsafe { std::ffi::CStr::from_ptr(key) };
	let key = key.to_str().expect("key is not a valid utf8 string");
	match self.get(key) {
	    Some(value) => value,
	    None => std::ptr::null(),
	}
    }

    #[no_mangle]
    pub extern "C" fn insert_value(&mut self, key: *const c_char, value: *const Value) {
	let key = unsafe { std::ffi::CStr::from_ptr(key) };
	let key = key.to_str().expect("key is not a valid utf8 string").to_string();
	let value = unsafe { &*value };
	self.insert(key, value.clone());
    }

    #[no_mangle]
    pub extern "C" fn len(&self) -> usize {
	self.keywords.len()
    }

    #[no_mangle]
    pub extern "C" fn is_empty(&self) -> bool {
	self.keywords.is_empty()
    }
}
