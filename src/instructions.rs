use crate::{
    error::Error,
    tokens::{Token, TokenType},
};
use Instruction::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    Add(u8),
    Subtract(u8),
    MoveRight(usize),
    MoveLeft(usize),
    Input,
    Output,
    LoopStart(usize),
    LoopEnd(usize),
}

impl Instruction {
    pub fn from_tokens<T: Iterator<Item = Token>>(tokens: T) -> Result<Vec<Instruction>, Error> {
        let instructions = tokens.map(|token| match token.ty {
            TokenType::Increment => Add(1),
            TokenType::Decrement => Subtract(1),
            TokenType::MoveRight => MoveRight(1),
            TokenType::MoveLeft => MoveLeft(1),
            TokenType::Input => Input,
            TokenType::Output => Output,
            // Set source position until the actual matching token is computed.
            TokenType::LoopStart => LoopStart(token.pos),
            TokenType::LoopEnd => LoopEnd(token.pos),
        });
        let mut instructions = Self::squash_arithmetic(instructions);
        Self::optimize_loops(&mut instructions)?;
        Ok(instructions)
    }

    fn squash_arithmetic<T: Iterator<Item = Instruction>>(instructions: T) -> Vec<Instruction> {
        let mut squashed = Vec::new();
        let mut sum: i32 = 0;
        for instruction in instructions {
            match instruction {
                Add(x) => sum += x as i32,
                Subtract(x) => sum -= x as i32,
                _ => {
                    if sum > 0 {
                        squashed.push(Add((sum % 256) as u8));
                    } else if sum < 0 {
                        squashed.push(Subtract((-sum % 256) as u8));
                    }
                    sum = 0;
                    squashed.push(instruction);
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
        if let Some((_, src_pos)) = stack.pop() {
            return Err(Error::UnbalancedLoopError('[', src_pos + 1));
        }
        for ((start, _), end) in pairs {
            instructions[start] = LoopStart(end);
            instructions[end] = LoopEnd(start);
        }
        Ok(())
    }
}
