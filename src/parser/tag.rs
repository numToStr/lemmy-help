use chumsky::select;

use crate::{impl_parse, TagType};

#[derive(Debug)]
pub struct Tag(String);

impl_parse!(Tag, {
    select! { TagType::Tag(x) => Self(x) }
});
