use std::fmt::Display;

use chumsky::select;

use crate::{lexer::TagType, parser::impl_parse};

#[derive(Debug, Clone)]
pub struct Divider(pub char);

impl_parse!(Divider, {
    select! { TagType::Divider(rune) => Self(rune) }
});

impl Display for Divider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0.to_string().repeat(80))
    }
}
