use std::collections::HashMap;
use crate::interpreter::value::Value;


#[repr(C)]
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

    pub fn len(&self) -> usize {
	self.keywords.len()
    }

    pub fn is_empty(&self) -> bool {
	self.keywords.is_empty()
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
    
}
