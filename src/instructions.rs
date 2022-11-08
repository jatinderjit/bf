use crate::tokens::Token;
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
    pub fn from_tokens(tokens: &[Token]) -> Vec<Instruction> {
        let instructions = tokens.iter().filter_map(|token| match token {
            Token::Increment => Some(Add(1)),
            Token::Decrement => Some(Subtract(1)),
            Token::MoveRight => Some(MoveRight(1)),
            Token::MoveLeft => Some(MoveLeft(1)),
            Token::Input => Some(Input),
            Token::Output => Some(Output),
            // Set an invalid value until the actual matching tokens are
            // computed.
            Token::LoopStart => Some(LoopStart(tokens.len())),
            Token::LoopEnd => Some(LoopEnd(tokens.len())),
            Token::Comment => None,
        });
        let mut instructions = Self::squash_arithmetic(instructions);
        Self::optimize_loops(&mut instructions);
        instructions
    }

    fn squash_arithmetic(instructions: impl Iterator<Item = Instruction>) -> Vec<Instruction> {
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

    fn optimize_loops(instructions: &mut Vec<Instruction>) {
        let mut stack = Vec::new();
        let mut pairs = Vec::new();
        for (i, instruction) in instructions.iter().enumerate() {
            match instruction {
                LoopStart(_) => stack.push(i),
                LoopEnd(_) => {
                    let start = stack.pop().expect("Invalid loop closure");
                    pairs.push((start, i));
                }
                _ => {}
            }
        }
        if !stack.is_empty() {
            panic!("Unclosed loop");
        }
        for (start, end) in pairs {
            instructions[start] = LoopStart(end);
            instructions[end] = LoopEnd(start);
        }
    }
}
