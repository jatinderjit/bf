mod error;
mod instructions;
mod tokens;

pub use error::Error;
use instructions::Instruction;
use std::io::{stdin, stdout, Read, Write};
use tokens::Token;

const RAM_SIZE: usize = 30000;

pub struct Program {
    memory: [u8; RAM_SIZE],
    pointer: usize,
    program_counter: usize,
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn compile(source: &[u8]) -> Result<Self, Error> {
        let tokens = source
            .iter()
            .enumerate()
            .filter_map(|(pos, char)| Token::try_from((pos, *char)).ok());
        let instructions = Instruction::from_tokens(tokens)?;
        Ok(Self {
            memory: [0; RAM_SIZE],
            pointer: 0,
            program_counter: 0,
            instructions,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        use instructions::Instruction::*;

        loop {
            if self.program_counter == self.instructions.len() {
                return Ok(());
            }
            match self.instructions[self.program_counter] {
                Input => stdin().read_exact(&mut self.memory[self.pointer..self.pointer + 1])?,
                Output => {
                    stdout().write_all(&self.memory[self.pointer..self.pointer + 1])?;
                }
                Add(num) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(num);
                }
                Subtract(num) => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(num);
                }
                MoveRight(jump) => {
                    self.pointer += jump;
                    assert!(self.pointer < RAM_SIZE);
                }
                MoveLeft(jump) => {
                    assert!(self.pointer >= jump);
                    self.pointer -= jump;
                }
                LoopStart(end) => {
                    if self.memory[self.pointer] == 0 {
                        self.program_counter = end;
                    }
                }
                LoopEnd(start) => {
                    if self.memory[self.pointer] > 0 {
                        self.program_counter = start;
                    }
                }
            };
            self.program_counter += 1;
        }
    }
}
