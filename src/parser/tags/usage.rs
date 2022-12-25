use chumsky::{
    primitive::{choice, just},
    select, Parser,
};

use crate::{lexer::TagType, parser::impl_parse};

#[derive(Debug, Clone)]
pub struct Usage {
    pub lang: Option<String>,
    pub code: String,
}

impl_parse!(Usage, {
    choice((
        select! {
            TagType::UsageStart(lang) => lang
        }
        .then(select! { TagType::Comment(x) => x }.repeated())
        .then_ignore(just(TagType::UsageEnd))
        .map(|(lang, code)| Self {
            lang,
            code: code.join("\n"),
        }),
        select! {
            TagType::Usage(lang, code) => Self { lang, code }
        },
    ))
});
