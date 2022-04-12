use std::fmt::Display;

use chumsky::select;

use crate::{asterisk, impl_parse, TagType};

#[derive(Debug)]
pub struct Tag(String);

impl_parse!(Tag, {
    select! { TagType::Tag(x) => Self(x) }
});

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = asterisk(&self.0);

        writeln!(f, "{}{}", " ".repeat(80 - t.len()), t)
    }
}
