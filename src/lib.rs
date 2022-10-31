mod tokens;

use std::io::{self, stdin, stdout, Read, Write};
use tokens::Token;

const RAM_SIZE: usize = 30000;

pub type Error = io::Error;

pub struct Program {
    memory: [u8; RAM_SIZE],
    pointer: usize,
    program_counter: usize,
    tokens: Vec<Token>,
}

impl Program {
    pub fn new(source: &[u8]) -> Self {
        Self {
            memory: [0; RAM_SIZE],
            pointer: 0,
            program_counter: 0,
            tokens: source.iter().map(|c| (*c).into()).collect(),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        use tokens::Token::*;

        loop {
            if self.program_counter == self.tokens.len() {
                return Ok(());
            }
            match self.tokens[self.program_counter] {
                Input => stdin().read_exact(&mut self.memory[self.pointer..self.pointer + 1])?,
                Output => {
                    stdout().write(&self.memory[self.pointer..self.pointer + 1])?;
                }
                Increment => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
                }
                Decrement => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
                }
                MoveRight => {
                    self.pointer += 1;
                    assert!(self.pointer < RAM_SIZE);
                }
                MoveLeft => {
                    assert!(self.pointer != 0);
                    self.pointer -= 1;
                }
                LoopStart => {
                    if self.memory[self.pointer] == 0 {
                        let mut loops: usize = 1;
                        // TODO: handle invalid loops
                        while loops > 0 {
                            self.program_counter += 1;
                            if self.tokens[self.program_counter] == LoopStart {
                                loops += 1;
                            } else if self.tokens[self.program_counter] == LoopEnd {
                                loops -= 1;
                            }
                        }
                    }
                }
                LoopEnd => {
                    if self.memory[self.pointer] > 0 {
                        let mut loops: usize = 1;
                        // TODO: handle invalid loops
                        while loops > 0 {
                            self.program_counter -= 1;
                            if self.tokens[self.program_counter] == LoopStart {
                                loops -= 1;
                            } else if self.tokens[self.program_counter] == LoopEnd {
                                loops += 1;
                            }
                        }
                    }
                }
                Comment => {}
            };
            self.program_counter += 1;
        }
    }
}
