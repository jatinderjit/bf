use std::io::{self, stdin, stdout, Read, Write};

const RAM_SIZE: usize = 30000;

pub type Error = io::Error;

pub struct Program {
    memory: [u8; RAM_SIZE],
    pointer: usize,
    program_counter: usize,
    source: Vec<u8>,
}

impl Program {
    pub fn new(source: Vec<u8>) -> Self {
        Self {
            memory: [0; RAM_SIZE],
            pointer: 0,
            program_counter: 0,
            source,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            if self.program_counter == self.source.len() {
                return Ok(());
            }
            match self.source[self.program_counter] {
                b',' => stdin().read_exact(&mut self.memory[self.pointer..self.pointer + 1])?,
                b'.' => {
                    stdout().write(&self.memory[self.pointer..self.pointer + 1])?;
                }
                b'+' => self.memory[self.pointer] += 1, //TODO: handle underflow
                b'-' => self.memory[self.pointer] -= 1, //TODO: handle overflow
                b'>' => self.pointer += 1,              // TODO: handle memory overflow
                b'<' => {
                    if self.pointer == 0 {
                        self.pointer = self.source.len() - 1;
                    } else {
                        self.pointer -= 1;
                    }
                }
                b'[' => {
                    if self.memory[self.pointer] == 0 {
                        let mut loops: usize = 1;
                        // TODO: handle invalid loops
                        while loops > 0 {
                            self.program_counter += 1;
                            if self.source[self.program_counter] == b'[' {
                                loops += 1;
                            } else if self.source[self.program_counter] == b']' {
                                loops -= 1;
                            }
                        }
                    } else {
                        self.program_counter += 1;
                    }
                }
                b']' => {
                    let mut loops: usize = 1;
                    // TODO: handle invalid loops
                    while loops > 0 {
                        self.program_counter -= 1;
                        if self.source[self.program_counter] == b'[' {
                            loops -= 1;
                        } else if self.source[self.program_counter] == b']' {
                            loops += 1;
                        }
                    }
                }
                _ => {}
            };
            self.program_counter += 1;
        }
    }
}
