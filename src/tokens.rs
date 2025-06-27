#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    Input,
    Output,
    LoopStart,
    LoopEnd,
}

pub struct Comment;

impl TryFrom<u8> for Token {
    type Error = Comment;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Token::*;

        match value {
            b'+' => Ok(Increment),
            b'-' => Ok(Decrement),
            b'>' => Ok(MoveRight),
            b'<' => Ok(MoveLeft),
            b',' => Ok(Input),
            b'.' => Ok(Output),
            b'[' => Ok(LoopStart),
            b']' => Ok(LoopEnd),
            _ => Err(Comment),
        }
    }
}
