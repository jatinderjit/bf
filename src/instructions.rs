use crate::{
    error::Error,
    tokens::{Token, TokenType},
};
use Instruction::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    Add(u8),
    Jump(isize),
    Input,
    Output,
    LoopStart(usize),
    LoopEnd(usize),
}

impl Instruction {
    pub fn from_tokens<T: Iterator<Item = Token>>(tokens: T) -> Result<Vec<Instruction>, Error> {
        let instructions = tokens.map(|token| match token.ty {
            TokenType::Increment => Add(1),
            TokenType::Decrement => Add(255u8), // since it wraps!
            TokenType::MoveRight => Jump(1),
            TokenType::MoveLeft => Jump(-1),
            TokenType::Input => Input,
            TokenType::Output => Output,
            // Set source position until the actual matching token is computed.
            TokenType::LoopStart => LoopStart(token.pos),
            TokenType::LoopEnd => LoopEnd(token.pos),
        });
        let mut instructions = Self::squash_instructions(instructions);
        Self::optimize_loops(&mut instructions)?;
        Ok(instructions)
    }

    fn squash_instructions<T: Iterator<Item = Instruction>>(instructions: T) -> Vec<Instruction> {
        use Instruction::*;

        let mut squashed = Vec::new();
        let mut last = Input; // Anything apart from `Add`
        for instruction in instructions {
            match (instruction, last) {
                (Add(x), Add(y)) => {
                    last = Add(x.wrapping_add(y));

                    // We can safely unwrap, because we know that the last instruction
                    // was an `Add` instruction. Hence this is not empty.
                    *squashed.last_mut().unwrap() = last;
                }
                (Jump(x), Jump(y)) => {
                    last = Jump(x + y);
                    *squashed.last_mut().unwrap() = last;
                }
                _ => {
                    squashed.push(instruction);
                    last = instruction;
                }
            }
        }
        squashed
    }

    /// This should be called after `squash_arithmetic` (or other optimization
    /// that could change the size of the instructions vector)
    fn optimize_loops(instructions: &mut [Instruction]) -> Result<(), Error> {
        let mut stack = Vec::new();
        let mut pairs = Vec::new();
        for (i, instruction) in instructions.iter().enumerate() {
            match instruction {
                LoopStart(src_pos) => stack.push((i, *src_pos)),
                LoopEnd(src_pos) => {
                    let start = stack
                        .pop()
                        .ok_or(Error::UnbalancedLoopError(']', src_pos + 1))?;
                    pairs.push((start, i));
                }
                _ => {}
            }
        }
        if let Some((_, src_pos)) = stack.first() {
            return Err(Error::UnbalancedLoopError('[', src_pos + 1));
        }
        for ((start, _), end) in pairs {
            instructions[start] = LoopStart(end);
            instructions[end] = LoopEnd(start);
        }
        Ok(())
    }
}
