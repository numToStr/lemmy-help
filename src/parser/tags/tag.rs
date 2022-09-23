use std::fmt::Display;

use chumsky::select;

use crate::{lexer::TagType, parser::impl_parse};

#[derive(Debug, Clone)]
pub struct Tag(String);

impl_parse!(Tag, {
    select! { TagType::Tag(x) => Self(x) }
});

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>80}", format!("*{}*", self.0))
    }
}
