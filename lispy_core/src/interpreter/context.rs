use std::collections::{HashMap, HashSet};
use std::ffi::c_char;
use crate::gc::{self, Gc};
use crate::interpreter::value::Value;
use crate::interpreter::module::Module;
use crate::parser::r#macro::Macro;
use crate::stdlib::get_stdlib;
use std::sync::{Arc, RwLock, RwLockWriteGuard, TryLockError};
use std::sync::mpsc::Sender;
use std::cell::RefCell;


use super::{HelperResult, Exception};
use super::value::GcValue;

#[derive(Debug, Clone)]
pub struct ContextFrame {
    pub bindings: HashMap<String, Value>,
}

impl ContextFrame {
    pub fn new() -> Self {
	ContextFrame {
	    bindings: HashMap::new(),
	}
    }
    pub fn new_with_bindings(bindings: HashMap<String, Value>) -> Self {
	ContextFrame {
	    bindings,
	}
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
	let value = self.bindings.get(name);
        value
    }

    pub fn mark(&self) {
	for (_, value) in self.bindings.iter() {
	    value.mark();
	}
    }

    pub fn unmark(&self) {
	for (_, value) in self.bindings.iter() {
	    value.unmark();
	}
    }

    pub fn protect(&self) {
	for (_, value) in self.bindings.iter() {
	    value.protect();
	}
    }

    pub fn merge_frame(&mut self, frame: ContextFrame) {
	for (name, value) in frame.bindings {
	    self.bindings.insert(name, value);
	}
    }

    pub fn rebind(&mut self, name: &str, value: Value) {
	self.bindings.insert(name.to_string(), value);
    }
}

pub struct Context {
    gc_lock: Arc<RwLock<()>>,
    sender: Sender<Gc<GcValue>>,
    //modules: RefCell<HashMap<String, Module>>,
    frames: Vec<ContextFrame>,
    type_table: Arc<RwLock<Vec<Value>>>,
    symbols_to_table: Arc<RwLock<HashMap<Vec<String>, usize>>>,
    enum_idicies: Arc<RwLock<HashSet<usize>>>,
    macros: Arc<RwLock<HashSet<Macro>>>,
    dynamic_libraries: Arc<RwLock<Vec<libloading::Library>>>,
    files_to_modules: Arc<RwLock<HashMap<String, usize>>>,
    paths_to_modules: Arc<RwLock<HashMap<Vec<String>, usize>>>,
    modules: Arc<RwLock<Vec<Module>>>,
}

impl Context {
    pub fn new(gc_lock: Arc<RwLock<()>>, sender: Sender<Gc<GcValue>>, macros: HashSet<Macro>) -> Self {
	let mut ctx = Context {
	    gc_lock,
	    sender,
	    //modules: RefCell::new(HashMap::new()),
	    frames: vec![],
	    type_table: Arc::new(RwLock::new(Vec::new())),
	    symbols_to_table: Arc::new(RwLock::new(HashMap::new())),
	    enum_idicies: Arc::new(RwLock::new(HashSet::new())),
	    macros: Arc::new(RwLock::new(macros)),
	    dynamic_libraries: Arc::new(RwLock::new(Vec::new())),
	    files_to_modules: Arc::new(RwLock::new(HashMap::new())),
	    paths_to_modules: Arc::new(RwLock::new(HashMap::new())),
	    modules: Arc::new(RwLock::new(Vec::new())),
	};
	let string_name = Value::new_symbol(vec!["string".to_string()], &mut ctx);
	let integer_name = Value::new_symbol(vec!["integer".to_string()], &mut ctx);
	let float_name = Value::new_symbol(vec!["float".to_string()], &mut ctx);
	let boolean_name = Value::new_symbol(vec!["boolean".to_string()], &mut ctx);
	let symbol_name = Value::new_symbol(vec!["symbol".to_string()], &mut ctx);
	let list_name = Value::new_symbol(vec!["list".to_string()], &mut ctx);
	let vector_name = Value::new_symbol(vec!["vector".to_string()], &mut ctx);
	let function_name = Value::new_symbol(vec!["procedure".to_string()], &mut ctx);
	let nil_name = Value::new_symbol(vec!["nil".to_string()], &mut ctx);
	let sexpr_name = Value::new_symbol(vec!["sexpr".to_string()], &mut ctx);
	let char_name = Value::new_symbol(vec!["char".to_string()], &mut ctx);
	let rust_value_name = Value::new_symbol(vec!["rust-value".to_string()], &mut ctx);
	let bytevector_name = Value::new_symbol(vec!["bytevector".to_string()], &mut ctx);
	let c_value_name = Value::new_symbol(vec!["c-value".to_string()], &mut ctx);
	let type_table = vec![nil_name, string_name, integer_name, float_name, boolean_name, symbol_name, list_name, vector_name, function_name, char_name, sexpr_name, rust_value_name, bytevector_name, c_value_name];
	type_table.iter().for_each(|v| v.protect());
	ctx.type_table = Arc::new(RwLock::new(type_table));
	
	let stdlib = get_stdlib(&mut ctx);
	ctx.frames.push(stdlib);

	let thread = crate::stdlib::thread::get_thread_library(&mut ctx);
	ctx.add_module("thread", thread);

	let file = crate::stdlib::file::get_file_library(&mut ctx);
	ctx.add_module("file", file);

	let network = crate::stdlib::network::get_network_library(&mut ctx);
	ctx.add_module("network", network);

	let sync = crate::stdlib::sync::get_sync_library(&mut ctx);
	ctx.add_module("sync", sync);
	
	ctx
    }
    pub fn new_no_type_table(gc_lock: Arc<RwLock<()>>, sender: Sender<Gc<GcValue>>) -> Self {
	let mut ctx = Context {
	    gc_lock,
	    sender,
	    //modules: RefCell::new(HashMap::new()),
	    frames: vec![],
	    type_table: Arc::new(RwLock::new(Vec::new())),
	    symbols_to_table: Arc::new(RwLock::new(HashMap::new())),
	    enum_idicies: Arc::new(RwLock::new(HashSet::new())),
	    macros: Arc::new(RwLock::new(HashSet::new())),
	    dynamic_libraries: Arc::new(RwLock::new(Vec::new())),
	    files_to_modules: Arc::new(RwLock::new(HashMap::new())),
	    paths_to_modules: Arc::new(RwLock::new(HashMap::new())),
	    modules: Arc::new(RwLock::new(Vec::new())),

	};
	
	let stdlib = get_stdlib(&mut ctx);
	ctx.frames.push(stdlib);

	let thread = crate::stdlib::thread::get_thread_library(&mut ctx);
	ctx.add_module("thread", thread);

	let file = crate::stdlib::file::get_file_library(&mut ctx);
	ctx.add_module("file", file);

	let network = crate::stdlib::network::get_network_library(&mut ctx);
	ctx.add_module("network", network);

	let sync = crate::stdlib::sync::get_sync_library(&mut ctx);
	ctx.add_module("sync", sync);
	
	ctx
    }

    pub fn push_frame(&mut self, frame: Option<ContextFrame>) {
	match frame {
	    Some(frame) => self.frames.push(frame),
	    None => self.frames.push(ContextFrame { bindings: HashMap::new() }),
	}
    }

    pub fn pop_frame(&mut self) -> Option<ContextFrame> {
	self.frames.pop()
    }

    pub fn copy_frame(&mut self) -> ContextFrame {
	let frame = self.frames.last().unwrap().clone();
	frame
    }

    pub fn copy_frame_at(&self, pos: usize) -> ContextFrame {
	self.frames[pos].clone()
    }

    fn get_from_frame(&self, name: &str) -> Option<&Value> {
        for frame in self.frames.iter().rev() {
            if let Some(value) = frame.get(name) {
                return Some(value);
            }
        }
    	None
    }

    pub fn is_bound(&self, name: &Vec<String>) -> bool {
	self.get(name.clone()).is_some()
    }

    fn lookup_module_in_path(&self, name: &[String]) -> Option<usize> {
	let paths_to_modules = self.paths_to_modules.clone();
	let paths_to_modules = paths_to_modules.read().unwrap();
	let mut module = paths_to_modules.get(name);
	if module.is_some() {
	    return module.cloned();
	}
	None
    }

    pub fn get(&self, name: Vec<String>) -> Option<Value> {
	//println!("get: {:?}", name);

	let value = self.get_from_frame(&name.last().unwrap());
	if value.is_some() {
	    return value.cloned();
	}

	let mut slice_index = 0;

	while slice_index < name.len() {
	    let module_index = self.lookup_module_in_path(&name.as_slice()[slice_index..name.len() - 1]);
	    let Some(module_index) = module_index else {
		slice_index += 1;
		continue;
	    };

	    let Some(module) = self.get_module(module_index) else {
		slice_index += 1;
		continue;
	    };

	    let value = module.get(&name.last().unwrap(), self);
	    if value.is_some() {
		return value;
	    }

	    slice_index += 1;
	}
	
	None
    }

    pub fn define(&mut self, name: &str, value: Value) {
	self.frames.last_mut().unwrap().bindings.insert(name.to_string(), value);
    }

    pub fn bind(&mut self, name: &Vec<String>, value: Value) {
	if name.len() == 1 {
	    self.define(&name[0], value);
	} else {
	    //if let Some(_) = self.modules.borrow_mut().get_mut(&name[0]) {
		todo!("bind to module");
		//module.bind(&name.as_slice()[1..], value, self);
	    //}
	}
    }

    pub fn rebind(&mut self, name: &Vec<String>, value: Value) {
	if name.len() == 1 {
	    for frame in self.frames.iter_mut().rev() {
		if frame.bindings.contains_key(&name[0]) {
		    frame.bindings.insert(name[0].to_string(), value);
		    return;
		}
	    }
	}
    }

    pub fn add_module(&mut self, name: &str, module: Module) {
	let path = vec![name.to_string()];

	let index = self.modules.read().unwrap().len();
	self.modules.write().unwrap().push(module);

	self.paths_to_modules.write().unwrap().insert(path, index);
    }

    /// TODO: add way to look on load path (LISPY_LOAD_PATH)
    pub fn add_module_from_file(&mut self, name: &str, path: Vec<String>) -> HelperResult<()> {
	let file_path = std::path::Path::new(name);
	let file_path = file_path.canonicalize()
	    .map_err(|err| Box::new(Exception::new(&vec!["import"], &format!("{}", err), self)))?;

	let file_path_str = file_path.clone();
	let file_path_str = file_path_str.to_str()
	    .ok_or(Box::new(Exception::new(&vec!["import"], "path is invalid string", self)))?;
	
	if let Some(index) = self.files_to_modules.read().unwrap().get(file_path_str) {
	    self.paths_to_modules.write().unwrap().insert(path, *index);
	    return Ok(());
	}

	if !file_path.exists() {
	    return Err(Box::new(Exception::new(&vec!["import"], "file does not exist", self)));
	}
	if !file_path.is_file() {
	    return Err(Box::new(Exception::new(&vec!["import"], "not a file", self)));
	}

	let module = match file_path.as_path().extension().map(|ext| ext.to_str().unwrap()) {
	    Some("so") | Some("dll") | Some("dylib") => {
		println!("\nLoading dynamic lib\n");
		crate::ffi::load_dynamic_lib(self, file_path_str)
		    .map_err(|err| Box::new(Exception::new(&vec!["import"], &format!("{}", err), self)))?
	    }
	    _ => {
		Module::new(file_path_str, path.clone())
	    }
	};

	let index = self.modules.read().unwrap().len();
	self.modules.write().unwrap().push(module);

	self.files_to_modules.write().unwrap().insert(file_path_str.to_string(), index);
	let mut slice_index = 0;
	while slice_index < path.len() {
	    self.paths_to_modules.write().unwrap().insert(path.as_slice()[slice_index..].to_vec(), index);
	    slice_index += 1;
	}
	//self.paths_to_modules.write().unwrap().insert(path, index);

	Ok(())
    }

    fn get_module(&self, index: usize) -> Option<Module> {
	self.modules.read().unwrap().get(index).cloned()
    }
    
    pub fn garbage_collect(&mut self) {
	let lock = self.gc_lock.read().unwrap();
	for frame in self.frames.iter_mut() {
	    frame.mark();
	}
	match self.modules.try_write() {
	    Ok(mut modules) => {
		for module in modules.iter_mut() {
		    module.mark();
		}
	    },
	    Err(TryLockError::WouldBlock) => {},
	    Err(TryLockError::Poisoned(_)) => {},
	}
	match self.type_table.try_write() {
	    Ok(mut type_table) => {
		for value in type_table.iter_mut() {
		    value.mark();
		}
	    },
	    Err(TryLockError::WouldBlock) => {},
	    Err(TryLockError::Poisoned(_)) => {},
	}
	drop(lock);
	while gc::is_gc_on() {

	}
	for frame in self.frames.iter_mut() {
	    frame.unmark();
	}
	match self.modules.try_write() {
	    Ok(mut modules) => {
		for module in modules.iter_mut() {
		    module.unmark();
		}
	    },
	    Err(TryLockError::WouldBlock) => {},
	    Err(TryLockError::Poisoned(_)) => {},
	}
	match self.type_table.try_write() {
	    Ok(mut type_table) => {
		for value in type_table.iter_mut() {
		    value.unmark();
		}
	    },
	    Err(TryLockError::WouldBlock) => {},
	    Err(TryLockError::Poisoned(_)) => {},
	}
	
    }
    
    pub fn garbage_collect_vm(&mut self, stack: &mut Vec<Value>) {
	let lock = self.gc_lock.read().unwrap();
	for frame in self.frames.iter_mut() {
	    frame.mark();
	}

	match self.modules.try_write() {
	    Ok(mut modules) => {
		for module in modules.iter_mut() {
		    module.mark();
		}
	    },
	    Err(TryLockError::WouldBlock) => {},
	    Err(TryLockError::Poisoned(_)) => {},
	}
	match self.type_table.try_write() {
	    Ok(mut type_table) => {
		for value in type_table.iter_mut() {
		    value.mark();
		}
	    },
	    Err(TryLockError::WouldBlock) => {},
	    Err(TryLockError::Poisoned(_)) => {},
	}
	for value in stack.iter() {
	    value.mark();
	}
	
	drop(lock);
	while gc::is_gc_on() {

	}
	for frame in self.frames.iter_mut() {
	    frame.unmark();
	}
	match self.modules.try_write() {
	    Ok(mut modules) => {
		for module in modules.iter_mut() {
		    module.unmark();
		}
	    },
	    Err(TryLockError::WouldBlock) => {},
	    Err(TryLockError::Poisoned(_)) => {},
	}
	match self.type_table.try_write() {
	    Ok(mut type_table) => {
		for value in type_table.iter_mut() {
		    value.unmark();
		}
	    },
	    Err(TryLockError::WouldBlock) => {},
	    Err(TryLockError::Poisoned(_)) => {},
	}
	for value in stack.iter() {
	    value.unmark();
	}
	
    }

    pub fn send_gc(&self, gc: Gc<GcValue>) {
	self.sender.send(gc).unwrap();
    }

    pub fn get_type_symbol(&self, index: usize) -> Value {
	self.type_table.read().unwrap()[index].clone()
    }

    pub fn get_or_create_type_symbol(&self, name: &Vec<String>) -> usize {
	let read_lock = self.symbols_to_table.read().unwrap();
	if let Some(index) = read_lock.get(name) {
	    *index
	} else {
	    drop(read_lock);
	    let mut type_table = self.type_table.write().unwrap();
	    let index = type_table.len();
	    let symbol = Value::new_symbol(name.clone(), self);
	    type_table.push(symbol.clone());
	    self.symbols_to_table.write().unwrap().insert(name.clone(), index);
	    let mut name = &name.as_slice()[1..];
	    while name.len() > 0 {
		self.symbols_to_table.write().unwrap().insert(name.to_vec(), index);
		name = &name[1..];
	    }
	    index
	}
    }
    pub fn get_or_create_type_symbol_enum(&self, name: &Vec<String>) -> usize {
	let read_lock = self.symbols_to_table.read().unwrap();
	if let Some(index) = read_lock.get(name) {
	    *index
	} else {
	    drop(read_lock);
	    let mut type_table = self.type_table.write().unwrap();
	    let index = type_table.len();
	    let symbol = Value::new_symbol(name.clone(), self);
	    type_table.push(symbol.clone());
	    self.symbols_to_table.write().unwrap().insert(name.clone(), index);
	    self.enum_idicies.write().unwrap().insert(index);
	    let mut name = &name.as_slice()[1..];
	    while name.len() > 0 {
		self.symbols_to_table.write().unwrap().insert(name.to_vec(), index);
		name = &name[1..];
	    }
	    index
	}
    }

    pub fn get_type_index(&self, name: &Vec<String>) -> Option<usize> {
	let read_lock = self.symbols_to_table.read().unwrap();
	let index = read_lock.get(name);
	index.cloned()
    }

    pub fn is_enum(&self, index: usize) -> bool {
	self.enum_idicies.read().unwrap().contains(&index)
    }

    pub fn get_macros(&self) -> RwLockWriteGuard<HashSet<Macro>> {
	self.macros.write().unwrap()
    }

    pub fn add_dynamic_lib(&self, lib: libloading::Library) {
	let mut dynamic_libraries = self.dynamic_libraries.write().unwrap();
	dynamic_libraries.push(lib);
    }

    /*pub fn copy_module_into_current(&self, module_path: &Vec<String>, name: &String) -> HelperResult<()> {
	match self.modules.borrow().get(&module_path[0]) {
	    None => Err(Box::new(Exception::new(&vec!["import-from"], "module not found", self))),
	    Some(module) => {
		let module = module
		    .get_submodule(&module_path.as_slice()[1..], self)
		    .ok_or(Box::new(Exception::new(&vec!["import-from"], "module path not found", self)))?;
		self.modules.borrow_mut().insert(name.clone(), module);
		Ok(())
	    }
	}
    }

    pub fn load_module_into_current(&mut self, module_path: &Vec<String>) -> HelperResult<()> {
	let module = match self.modules.borrow().get(&module_path[0]) {
	    None => return Err(Box::new(Exception::new(&vec!["import-from"], "module not found", self))),
	    Some(module) => {
		let module = module
		    .get_submodule(&module_path.as_slice()[1..], self)
		    .ok_or(Box::new(Exception::new(&vec!["import-from"], "module path not found", self)))?;
		module
	    }
	};
	
	let Some((submodules, frame)) = module.into_loaded() else {
	    return Err(Box::new(Exception::new(&vec!["import-from"], "module path not found", self)));
	};

	for (name, module) in submodules {
	    self.add_module(&name, module);
	}

	self.frames.last_mut().unwrap().merge_frame(frame);
	Ok(())
    }*/
	
}

// FFI functions for context
impl Context {
    #[no_mangle]
    pub extern "C" fn context_define(context: *mut Context, name: *const c_char, name_len: usize, value: *mut Value) {
	let mut str_buf: Vec<u8> = vec![0; name_len];
	for i in 0..name_len {
	    str_buf[i] = unsafe { *name.add(i) as u8 };// TODO: make this safe
	}
	let name = std::str::from_utf8(&str_buf).unwrap();
	let value = unsafe { value.as_ref() }.unwrap();
	let context = unsafe { context.as_mut() }.unwrap();
	context.define(name, value.clone());
    }

    #[no_mangle]
    pub extern "C" fn context_push_frame(context: *mut Context) {
	let context = unsafe { context.as_mut() }.unwrap();
	context.push_frame(None);
    }

    #[no_mangle]
    pub extern "C" fn context_pop_frame(context: *mut Context) -> *mut ContextFrame {
	let context = unsafe { context.as_mut() }.unwrap();
	let frame = context.pop_frame().unwrap();
	Box::into_raw(Box::new(frame))
    }

    #[no_mangle]
    pub extern "C" fn context_add_module(context: *mut Context, name: *const u8, name_len: usize, module: *mut Module) {
	let mut str_buf = vec![0; name_len];
	for i in 0..name_len {
	    str_buf[i] = unsafe { *name.add(i) };
	}
	let name = std::str::from_utf8(&str_buf).unwrap();
	let module = unsafe { module.as_ref() }.unwrap().clone();
	let context = unsafe { context.as_mut() }.unwrap();
	context.add_module(name, module);
    }

}


unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Clone for Context {
    fn clone(&self) -> Self {
	Context {
	    gc_lock: self.gc_lock.clone(),
	    sender: self.sender.clone(),
	    frames: Vec::new(),
	    type_table: self.type_table.clone(),
	    symbols_to_table: self.symbols_to_table.clone(),
	    enum_idicies: self.enum_idicies.clone(),
	    macros: self.macros.clone(),
	    dynamic_libraries: self.dynamic_libraries.clone(),
	    files_to_modules: self.files_to_modules.clone(),
	    paths_to_modules: self.paths_to_modules.clone(),
	    modules: self.modules.clone(),
	}
    }
}

impl Default for Context {
    fn default() -> Self {
	let (sender, _) = std::sync::mpsc::channel();
	Context::new_no_type_table(Arc::new(RwLock::new(())), sender)
    }
}
