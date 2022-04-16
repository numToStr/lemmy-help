use std::fmt::Display;

use chumsky::select;

use crate::{impl_parse, TagType};

#[derive(Debug, Clone)]
pub struct Tag(String);

impl_parse!(Tag, {
    select! { TagType::Tag(x) => Self(x) }
});

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}*{}*", " ".repeat(78 - &self.0.len()), &self.0)
    }
}
