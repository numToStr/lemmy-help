mod parser;
use std::{error::Error, fmt::Display, io};

use chumsky::prelude::Simple;
pub use parser::*;

pub type LemmyResult<T> = Result<T, LemmyError>;

#[derive(Debug)]
pub enum LemmyError {
    Io(io::Error),
    Lexer(Vec<Simple<char>>),
    Parser(Vec<Simple<TagType>>),
}

impl From<io::Error> for LemmyError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl Display for LemmyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::Lexer(e) => write!(f, "{:?}", e),
            Self::Parser(e) => write!(f, "{:?}", e),
        }
    }
}

impl Error for LemmyError {}
