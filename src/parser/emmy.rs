use std::ops::Range;

use chumsky::{
    prelude::{any, choice, end, filter, just, take_until, Simple},
    text::{ident, newline, TextParser},
    Parser,
};

use crate::Scope;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagType {
    Module {
        name: String,
        desc: Option<String>,
    },
    Divider(char),
    Func {
        prefix: Option<String>,
        name: String,
        scope: Scope,
    },
    Expr {
        prefix: Option<String>,
        name: String,
        scope: Scope,
    },
    Export(String),
    BriefStart,
    BriefEnd,
    Param {
        name: String,
        ty: String,
        desc: Option<String>,
    },
    Return {
        ty: String,
        name: Option<String>,
        desc: Option<String>,
    },
    Class(String, Option<String>),
    Field {
        name: String,
        ty: String,
        desc: Option<String>,
    },
    Alias {
        name: String,
        ty: String,
        desc: Option<String>,
    },
    Type(String, Option<String>),
    Tag(String),
    See(String),
    Usage(String),
    Comment(String),
    Empty,
}

type Spanned = (TagType, Range<usize>);

#[derive(Debug)]
pub struct Emmy;

impl Emmy {
    pub fn parse(src: &str) -> Result<Vec<Spanned>, Vec<Simple<char>>> {
        let comment = take_until(newline().or(end())).map(|(x, _)| x.iter().collect());

        let ty = filter(|x: &char| !x.is_whitespace())
            .repeated()
            .padded()
            .collect();

        let name = ident().padded();

        let desc = choice((
            end().to(None),
            just("---").rewind().to(None),
            comment.clone().map(Some),
        ));

        let dotted = choice((
            ident()
                .then(just('.').to(Scope::Dot).or(just(':').to(Scope::Colon)))
                .then(ident())
                .map(|((prefix, scope), name)| (Some(prefix), scope, name)),
            ident().map(|name| (None, Scope::Local, name)),
        ))
        .padded();

        let tags = just('@')
            .ignore_then(choice((
                just("mod")
                    .ignore_then(ty)
                    .then(desc.clone())
                    .map(|(name, desc)| TagType::Module { name, desc }),
                just("divider")
                    .ignore_then(any().padded())
                    .map(TagType::Divider),
                just("func")
                    .ignore_then(dotted.clone())
                    .then(comment.clone())
                    .padded()
                    .map(|((prefix, scope, name), _)| TagType::Func {
                        prefix,
                        name,
                        scope,
                    }),
                just("expr")
                    .ignore_then(dotted)
                    .then(comment.clone())
                    .padded()
                    .map(|((prefix, scope, name), _)| TagType::Expr {
                        prefix,
                        name,
                        scope,
                    }),
                just("export")
                    .ignore_then(ident().padded())
                    .then_ignore(end())
                    .map(TagType::Export),
                just("brief").ignore_then(
                    choice((
                        just("[[").to(TagType::BriefStart),
                        just("]]").to(TagType::BriefEnd),
                    ))
                    .padded(),
                ),
                just("param")
                    .ignore_then(ty) // I am using `ty` here because param can have `?`
                    .then(ty)
                    .then(desc.clone())
                    .map(|((name, ty), desc)| TagType::Param { name, ty, desc }),
                just("return")
                    .ignore_then(ty)
                    .then(name.or_not())
                    .then(desc.clone())
                    .map(|((ty, name), desc)| TagType::Return { ty, name, desc }),
                just("class")
                    .ignore_then(name)
                    .then(desc.clone())
                    .map(|(name, desc)| TagType::Class(name, desc)),
                just("field")
                    .ignore_then(name)
                    .then(ty)
                    .then(desc.clone())
                    .map(|((name, ty), desc)| TagType::Field { name, ty, desc }),
                just("alias")
                    .ignore_then(name)
                    .then(ty)
                    .then(desc.clone())
                    .map(|((name, ty), desc)| TagType::Alias { ty, name, desc }),
                just("type")
                    .ignore_then(name)
                    .then(desc)
                    .map(|(name, desc)| TagType::Type(name, desc)),
                just("tag")
                    .ignore_then(comment.clone().padded())
                    .map(TagType::Tag),
                just("see")
                    .ignore_then(comment.clone().padded())
                    .map(TagType::See),
                just("usage")
                    .ignore_then(
                        just('`')
                            .ignore_then(filter(|c| *c != '`').repeated())
                            .then_ignore(just('`'))
                            .padded(),
                    )
                    .collect()
                    .map(TagType::Usage),
            )))
            .or(newline().to(TagType::Empty))
            .or(comment.map(TagType::Comment));

        just("---")
            .ignore_then(tags)
            .map_with_span(|t, r| (t, r))
            .repeated()
            .parse(src)
    }
}
