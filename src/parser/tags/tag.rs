use std::fmt::Display;

use chumsky::select;

use crate::{parser, TagType};

#[derive(Debug, Clone)]
pub struct Tag(String);

parser!(Tag, {
    select! { TagType::Tag(x) => Self(x) }
});

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>80}", format!("*{}*", self.0))
    }
}
