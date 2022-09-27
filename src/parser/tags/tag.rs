use chumsky::select;

use crate::{lexer::TagType, parser::impl_parse};

#[derive(Debug, Clone)]
pub struct Tag(pub String);

impl_parse!(Tag, {
    select! { TagType::Tag(x) => Self(x) }
});
