extern crate lispy_core;

//pub mod parser;
//pub mod interpreter;
//pub mod stdlib;
//pub mod gc;
//pub mod ffi;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    lispy_core::run_from_file(&args[1], ".")?;
    Ok(())
}
