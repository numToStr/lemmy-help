use chumsky::{
    primitive::{choice, just},
    select, Parser,
};

use crate::{lexer::TagType, parser::impl_parse, Accept, Visitor};

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

impl<T: Visitor> Accept<T> for Usage {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.usage(self, s)
    }
}
