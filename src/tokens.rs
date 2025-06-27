#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub pos: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
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

impl TryFrom<(usize, u8)> for Token {
    type Error = Comment;

    fn try_from((pos, char): (usize, u8)) -> Result<Self, Self::Error> {
        use TokenType::*;

        let ty = match char {
            b'+' => Increment,
            b'-' => Decrement,
            b'>' => MoveRight,
            b'<' => MoveLeft,
            b',' => Input,
            b'.' => Output,
            b'[' => LoopStart,
            b']' => LoopEnd,
            _ => Err(Comment)?,
        };
        Ok(Token { ty, pos })
    }
}
