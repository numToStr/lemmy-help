use std::fmt::Display;

use chumsky::{
    primitive::{choice, just},
    select, IterParser, Parser,
};

use crate::{lexer::Token, parser::LemmyParser, Accept, Visitor};

#[derive(Debug, Clone)]
pub enum Code<'src> {
    InLine(&'src str),
    MultiLine(Vec<&'src str>),
}

impl<'src> Display for Code<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InLine(x) => f.write_str(x),
            Self::MultiLine(x) => f.write_str(&x.join("\n")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Usage<'src> {
    pub lang: Option<&'src str>,
    pub code: Code<'src>,
}

pub fn usage_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Usage<'src>> {
    choice((
        select! { Token::UsageStart(lang) => lang }
            .then(select! { Token::Comment(x) => x }.repeated().collect())
            .then_ignore(just(Token::UsageEnd))
            .map(|(lang, code)| Usage {
                lang,
                code: Code::MultiLine(code),
            }),
        select! { Token::Usage(lang,code) => Usage { lang,code:Code::InLine(code) } },
    ))
}

impl<'src, T: Visitor> Accept<T> for Usage<'src> {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.usage(self, s)
    }
}
