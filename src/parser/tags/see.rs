use chumsky::{select, Parser};

use crate::{lexer::TagType, parser::impl_parse};

#[derive(Debug, Clone)]
pub struct See {
    pub refs: Vec<String>,
}

impl_parse!(See, {
    select! { TagType::See(x) => x }
        .repeated()
        .map(|refs| Self { refs })
});
