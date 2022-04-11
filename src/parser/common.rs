use std::fmt::Display;

use chumsky::{
    prelude::{choice, end, filter, just, take_until},
    text::{self, Character, TextParser},
    Parser,
};

// Little helper macro for making parse function
#[macro_export]
macro_rules! impl_parse {
    ($id: ident, $body: expr) => {
        impl $id {
            pub fn parse(
            ) -> impl chumsky::Parser<char, Self, Error = chumsky::prelude::Simple<char>> {
                $body
            }
        }
    };
}

/// A TYPE could be
/// - primary = string|number|boolean
/// - fn = func(...):string
/// - enum = "one"|"two"|"three"
/// - or: primary (| primary)+
/// - optional = primary?
/// - table = table<string, string>
/// - array = primary[]
#[derive(Debug)]
pub struct Ty(pub String);

impl_parse!(Ty, {
    filter(|x: &char| !x.is_whitespace())
        .repeated()
        .padded()
        .collect()
        .map(Self)
});

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug)]
pub struct Name(pub String);

impl_parse!(Name, text::ident().padded().map(Self));

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// @comment
#[derive(Debug, Clone, Default)]
pub struct Desc(pub String);

impl Desc {
    pub fn parse(
    ) -> impl chumsky::Parser<char, Option<Self>, Error = chumsky::prelude::Simple<char>> {
        choice((
            end().to(None),
            just("---@").rewind().to(None),
            take_until(text::newline()).map(|(x, _)| Some(Self(x.into_iter().collect()))),
        ))
    }
}

impl Display for Desc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
