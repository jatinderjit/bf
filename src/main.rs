use brainfuck::Program;
use std::error::Error;
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = std::env::args()
        .nth(1)
        .ok_or("Please enter the file path")?;
    let source = std::fs::read(&file_name).map_err(|_| "Error reading file".to_string())?;
    let mut program = Program::compile(&source)?;
    program.run()?;
    Ok(())
}
