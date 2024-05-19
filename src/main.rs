pub mod parser;
pub mod interpreter;
pub mod stdlib;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = std::env::args().collect();

    let file_content = std::fs::read_to_string(&args[1])?;
    let file = parser::parse(&file_content)?;

    interpreter::walk_through::run(file)?;

    Ok(())
}
