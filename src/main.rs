use std::collections::HashSet;

pub mod parser;
pub mod interpreter;
pub mod stdlib;
pub mod gc;
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

    interpreter::walk_through::run(file, &mut context, &vec!["main".to_string()])?;
    
    tx.send(()).unwrap();
    std::process::exit(0);// Thi is needed due to threads that are not joined
    //Ok(())
}
