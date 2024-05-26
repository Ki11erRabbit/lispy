
pub mod interpreter;
pub mod gc;
pub mod parser;
pub mod stdlib;
pub mod ffi;

use std::collections::HashSet;
use interpreter::context::Context;
use interpreter::value::Value;
use interpreter::value::function::Function;
use interpreter::value::function::FunctionShape;
use interpreter::kwargs::Kwargs;
use interpreter::value::function::CFunctionOutput;



pub fn run_from_file(file_name: &str, so_load_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string(file_name)?;
    let mut macros = HashSet::new();
    let file = parser::parse(&file_content, &mut macros)?;

    let (tx, rx) = std::sync::mpsc::channel();
    let lock = std::sync::Arc::new(std::sync::RwLock::new(()));
    
    let gc_table = gc::GcTable::new(lock.clone(), rx);

    let mut context = interpreter::context::Context::new(lock, tx, macros);

    let (tx, rx) = std::sync::mpsc::channel();
    
    std::thread::spawn(move || {
	let mut gc_table = gc_table;
	gc::garbage_collect(&mut gc_table, rx);
    });

    //crate::ffi::get_ffi_library_test(&mut context)?;
    unsafe {
	let lib = libloading::Library::new("ffi.so")?;
	let func = lib.get::<unsafe extern "C" fn(*mut Context, *mut Value, usize, *mut Kwargs, *mut CFunctionOutput)>(b"hello_c")?;
	let shape = FunctionShape::new(vec![]);
	let function = Function::CNative(*func, shape);
	let function = Value::new_function(function, &mut context);
	context.define("hello-c", function);
	interpreter::walkthrough::run(file, &mut context, &vec!["main".to_string()])?;
    }

    //interpreter::walk_through::run(file, &mut context, &vec!["main".to_string()])?;
    
    tx.send(()).unwrap();
    std::process::exit(0);// Thi is needed due to threads that are not joined
    //Ok(())
}
