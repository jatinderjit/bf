#[derive(Debug, PartialEq)]
pub enum Token {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    Input,
    Output,
    LoopStart,
    LoopEnd,
    Comment,
}

impl From<u8> for Token {
    fn from(value: u8) -> Self {
        use Token::*;

        match value {
            b'+' => Increment,
            b'-' => Decrement,
            b'>' => MoveRight,
            b'<' => MoveLeft,
            b',' => Input,
            b'.' => Output,
            b'[' => LoopStart,
            b']' => LoopEnd,
            _ => Comment,
        }
    }
}
