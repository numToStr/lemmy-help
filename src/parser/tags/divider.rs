use std::fmt::Display;

use chumsky::select;

use crate::{parser, TagType};

#[derive(Debug, Clone)]
pub struct Divider(char);

parser!(Divider, {
    select! { TagType::Divider(rune) => Self(rune) }
});

impl Display for Divider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0.to_string().repeat(80))
    }
}
