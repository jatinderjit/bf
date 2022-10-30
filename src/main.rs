use brainfuck::{Error, Program};

fn main() -> Result<(), Error> {
    let mut source = vec![b'+'; 200];
    source[97] = b'>';
    source[197] = b'.';
    source[198] = b'<';
    source[199] = b'.'; // Should print 'ca'
    let mut program = Program::new(source);
    program.run()?;
    return Ok(());
}
