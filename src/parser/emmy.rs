use std::ops::Range;

use chumsky::{
    prelude::{any, choice, end, filter, just, take_until, Simple},
    text::{ident, keyword, newline, TextParser},
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
    Skip,
}

type Spanned = (TagType, Range<usize>);

#[derive(Debug)]
pub struct Emmy;

impl Emmy {
    pub fn parse(src: &str) -> Result<Vec<Spanned>, Vec<Simple<char>>> {
        let triple = just("---");

        let local = keyword("local").padded();

        let func = keyword("function").padded();

        let dotted = choice((
            ident()
                .then(just('.').to(Scope::Dot).or(just(':').to(Scope::Colon)))
                .then(ident())
                .map(|((prefix, scope), name)| (Some(prefix), scope, name)),
            ident().map(|name| (None, Scope::Local, name)),
        ))
        .padded();

        let expr = dotted.clone().padded().then_ignore(just('='));

        let comment = take_until(newline().or(end())).map(|(x, _)| x.iter().collect());

        let ty = filter(|x: &char| !x.is_whitespace()).repeated().collect();

        let name = ident().padded();

        let desc = choice((
            end().to(None),
            triple.rewind().to(None),
            comment.clone().map(Some),
        ));

        let private = just("private").then_ignore(
            choice((
                // eat up all the emmylua, if any, then one valid token
                triple
                    .then(take_until(newline().or(end())))
                    .padded()
                    .repeated()
                    .ignore_then(ident()),
                // if there is no emmylua, just eat the next token
                // so the next parser won't recognize the code
                ident(),
            ))
            .padded(),
        );

        let misc = take_until(newline());

        let tags = just('@')
            .ignore_then(choice((
                private.to(TagType::Skip),
                just("mod")
                    .ignore_then(ty.padded())
                    .then(desc.clone())
                    .map(|(name, desc)| TagType::Module { name, desc }),
                just("divider")
                    .ignore_then(any().padded())
                    .map(TagType::Divider),
                just("brief").ignore_then(
                    choice((
                        just("[[").to(TagType::BriefStart),
                        just("]]").to(TagType::BriefEnd),
                    ))
                    .padded(),
                ),
                just("param")
                    .ignore_then(ty.padded()) // I am using `ty` here because param can have `?`
                    .then(ty.padded())
                    .then(desc.clone())
                    .map(|((name, ty), desc)| TagType::Param { name, ty, desc }),
                just("return")
                    .ignore_then(
                        ty.then(choice((
                            newline().to((None, None)),
                            ident()
                                .then(newline().to(None).or(desc.clone().padded()))
                                .padded()
                                .map(|(name, desc)| (Some(name), desc)),
                        )))
                        .padded(),
                    )
                    .map(|(ty, (name, desc))| TagType::Return { ty, name, desc }),
                just("class")
                    .ignore_then(name)
                    .then(desc.clone())
                    .map(|(name, desc)| TagType::Class(name, desc)),
                just("field")
                    .ignore_then(name)
                    .then(ty.padded())
                    .then(desc.clone())
                    .map(|((name, ty), desc)| TagType::Field { name, ty, desc }),
                just("alias")
                    .ignore_then(name)
                    .then(ty.padded())
                    .then(desc.clone())
                    .map(|((name, ty), desc)| TagType::Alias { ty, name, desc }),
                just("type").ignore_then(
                    ty.then(choice((newline().to(None), desc.padded())))
                        .padded()
                        .map(|(name, desc)| TagType::Type(name, desc)),
                ),
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

        choice((
            triple.ignore_then(tags),
            local.ignore_then(choice((
                func.clone().ignore_then(ident()).map(|name| TagType::Func {
                    name,
                    prefix: None,
                    scope: Scope::Local,
                }),
                ident()
                    .padded()
                    .then_ignore(just('='))
                    .map(|name| TagType::Expr {
                        name,
                        prefix: None,
                        scope: Scope::Local,
                    }),
            ))),
            func.clone()
                .ignore_then(dotted)
                .map(|(prefix, scope, name)| TagType::Func {
                    prefix,
                    name,
                    scope,
                }),
            choice((
                expr.clone()
                    .then_ignore(func)
                    .map(|(prefix, scope, name)| TagType::Func {
                        prefix,
                        name,
                        scope,
                    }),
                expr.map(|(prefix, scope, name)| TagType::Expr {
                    prefix,
                    name,
                    scope,
                }),
            )),
            keyword("return")
                .ignore_then(ident().padded())
                .then_ignore(end())
                .map(TagType::Export),
            misc.to(TagType::Skip),
        ))
        .padded()
        .map_with_span(|t, r| (t, r))
        .repeated()
        .parse(src)
    }
}
