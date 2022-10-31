use brainfuck::{Error, Program};

fn main() -> Result<(), Error> {
    let file_name = std::env::args().nth(1).expect("Please enter the file path");
    let source = std::fs::read(&file_name).expect("Error reading file");
    let mut program = Program::new(&source);
    program.run()?;
    return Ok(());
}
