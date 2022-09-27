use chumsky::{
    primitive::{choice, just},
    select, Parser,
};

use crate::{lexer::TagType, parser::impl_parse};

#[derive(Debug, Clone)]
pub struct Usage {
    pub code: String,
}

impl_parse!(Usage, {
    choice((
        select! { TagType::Comment(x) => x }
            .repeated()
            .delimited_by(just(TagType::UsageStart), just(TagType::UsageEnd))
            .map(|x| Self { code: x.join("\n") }),
        select! { TagType::Usage(code) => Self { code } },
    ))
});
