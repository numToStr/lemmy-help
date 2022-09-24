use chumsky::select;

use crate::{lexer::TagType, parser::impl_parse};

#[derive(Debug, Clone)]
pub struct Divider(pub char);

impl_parse!(Divider, {
    select! { TagType::Divider(rune) => Self(rune) }
});
