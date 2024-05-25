use std::collections::HashSet;

use crate::interpreter::context::Context;
use crate::interpreter::value::Value;
use crate::interpreter::value::Function;
use crate::interpreter::value::FunctionShape;
use crate::interpreter::kwargs::Kwargs;
use crate::interpreter::value::CFunctionOutput;


pub mod parser;
pub mod interpreter;
pub mod stdlib;
pub mod gc;
pub mod ffi;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let file_content = std::fs::read_to_string(&args[1])?;
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
	interpreter::walk_through::run(file, &mut context, &vec!["main".to_string()])?;
    }

    //interpreter::walk_through::run(file, &mut context, &vec!["main".to_string()])?;
    
    tx.send(()).unwrap();
    std::process::exit(0);// Thi is needed due to threads that are not joined
    //Ok(())
}
