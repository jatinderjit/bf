use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unbalaced bracket '{0}' at position {1}")]
    UnbalancedLoopError(char, usize),

    #[error(transparent)]
    IO(#[from] io::Error),
}
