use std::ops::Range;

use chumsky::{
    prelude::{choice, end, filter, just, take_until},
    text::{self, TextParser},
    Parser,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagType {
    BriefStart,
    BriefEnd,
    Name(String),
    Param(Object),
    Return(Object),
    Class(String, Option<String>),
    Field(Object),
    Alias(Object),
    Type(String, Option<String>),
    Tag(String),
    See(String),
    Empty,
    Comment(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Object {
    pub ty: String,
    pub name: String,
    pub desc: Option<String>,
}

#[derive(Debug)]
pub struct Lexer;

impl Lexer {
    pub fn parse(
    ) -> impl chumsky::Parser<char, Vec<(TagType, Range<usize>)>, Error = chumsky::prelude::Simple<char>>
    {
        let comment = take_until(text::newline())
            // .padded()
            .map(|(x, _)| x.iter().collect());

        let ty = filter(|x: &char| !x.is_whitespace())
            .repeated()
            .padded()
            .collect();

        let name = text::ident().padded();

        let desc = choice((
            end().to(None),
            just("---").rewind().to(None),
            comment.map(Some),
        ));

        let tags = just('@')
            .ignore_then(choice((
                just("brief")
                    .ignore_then(just("[[").padded())
                    .to(TagType::BriefStart),
                just("brief")
                    .ignore_then(just("]]").padded())
                    .to(TagType::BriefEnd),
                just("name")
                    .ignore_then(comment.padded())
                    .map(TagType::Name),
                just("param")
                    .ignore_then(ty)
                    .then(name)
                    .then(desc.clone())
                    .map(|((ty, name), desc)| TagType::Param(Object { ty, name, desc })),
                just("return")
                    .ignore_then(ty)
                    .then(name)
                    .then(desc.clone())
                    .map(|((ty, name), desc)| TagType::Return(Object { ty, name, desc })),
                just("class")
                    .ignore_then(name)
                    .then(desc.clone())
                    .map(|(name, desc)| TagType::Class(name, desc)),
                just("field")
                    .ignore_then(ty)
                    .then(name)
                    .then(desc.clone())
                    .map(|((ty, name), desc)| TagType::Field(Object { ty, name, desc })),
                just("alias")
                    .ignore_then(name)
                    .then(ty)
                    .then(desc.clone())
                    .map(|((name, ty), desc)| TagType::Alias(Object { ty, name, desc })),
                just("type")
                    .ignore_then(name)
                    .then(desc)
                    .map(|(name, desc)| TagType::Type(name, desc)),
                just("tag").ignore_then(name).map(TagType::Tag),
                just("see").ignore_then(comment).map(TagType::See),
            )))
            .or(text::newline().to(TagType::Empty))
            .or(comment.map(TagType::Comment));

        just("---")
            .ignore_then(tags)
            .map_with_span(|t, r| (t, r))
            .repeated()
    }
}
