use std::ops::Range;

use chumsky::{
    prelude::{any, choice, end, filter, just, take_until, Simple},
    text::{ident, keyword, newline, whitespace, TextParser},
    Parser,
};

use crate::Kind;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scope {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagType {
    Toc(String),
    Module {
        name: String,
        desc: Option<String>,
    },
    Divider(char),
    Func {
        prefix: Option<String>,
        name: String,
        kind: Kind,
    },
    Expr {
        prefix: Option<String>,
        name: String,
        kind: Kind,
    },
    Export(String),
    BriefStart,
    BriefEnd,
    Param {
        name: String,
        ty: String,
        desc: Option<String>,
    },
    /// ---@return <type> [<name> [comment] | [name] #<comment>]
    Return {
        ty: String,
        name: Option<String>,
        desc: Option<String>,
    },
    Class(String, Option<String>),
    Field {
        scope: Scope,
        name: String,
        ty: String,
        desc: Option<String>,
    },
    Alias {
        name: String,
        ty: Option<String>,
        desc: Option<String>,
    },
    Variant(String, Option<String>),
    Type(String, Option<String>),
    Tag(String),
    See(String),
    Usage(String),
    UsageStart,
    UsageEnd,
    Comment(String),
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
                .then(choice((just('.').to(Kind::Dot), just(':').to(Kind::Colon))))
                .then(ident())
                .map(|((prefix, scope), name)| (Some(prefix), scope, name)),
            ident().map(|name| (None, Kind::Local, name)),
        ))
        .padded();

        let expr = dotted.clone().padded().then_ignore(just('='));

        let at_end = choice((newline(), end()));

        let comment = take_until(at_end.clone()).map(|(x, _)| x.iter().collect());

        let ty = filter(|x: &char| !x.is_whitespace()).repeated().collect();

        let name = ident().padded();

        let desc = choice((
            at_end.clone().to(None),
            triple.rewind().to(None),
            comment.clone().padded().map(Some),
        ));

        let scope = choice((
            keyword("public").to(Scope::Public),
            keyword("protected").to(Scope::Protected),
            keyword("private").to(Scope::Private),
        ))
        .padded();

        let private = just("private").then_ignore(
            choice((
                // eat up all the emmylua, if any, then one valid token
                triple
                    .then(take_until(at_end))
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

        let variant = just('|')
            .ignore_then(ty.padded())
            .then_ignore(just('#').or_not())
            .then(desc.clone())
            .map(|(t, d)| TagType::Variant(t, d));

        let tag = just('@').ignore_then(choice((
            private.to(TagType::Skip),
            just("toc")
                .then_ignore(whitespace())
                .ignore_then(comment.clone())
                .map(TagType::Toc),
            just("mod")
                .then_ignore(whitespace())
                .ignore_then(ty)
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
                .then(ty)
                .then(desc.clone())
                .map(|((name, ty), desc)| TagType::Param { name, ty, desc }),
            just("return")
                .ignore_then(whitespace())
                .ignore_then(ty)
                .then(choice((
                    newline().to((None, None)),
                    whitespace().ignore_then(choice((
                        just('#')
                            .ignore_then(comment.clone())
                            .map(|x| (None, Some(x))),
                        ident()
                            .then(desc.clone())
                            .map(|(name, desc)| (Some(name), desc)),
                    ))),
                )))
                .map(|(ty, (name, desc))| TagType::Return { ty, name, desc }),
            just("class")
                .ignore_then(name)
                .then(desc.clone())
                .map(|(name, desc)| TagType::Class(name, desc)),
            just("field")
                .ignore_then(scope.or_not())
                .then(name)
                .then(ty)
                .then(desc.clone())
                .map(|(((scope, name), ty), desc)| TagType::Field {
                    scope: scope.unwrap_or(Scope::Public),
                    name,
                    ty,
                    desc,
                }),
            just("alias")
                .then(whitespace())
                .ignore_then(ident())
                .then(choice((
                    newline().to((None, None)),
                    whitespace().ignore_then(ty.or_not().then(desc.clone())),
                )))
                .map(|(name, (ty, desc))| TagType::Alias { name, ty, desc }),
            just("type")
                .ignore_then(whitespace())
                .ignore_then(ty)
                .then(desc)
                .map(|(name, desc)| TagType::Type(name, desc)),
            just("tag")
                .ignore_then(comment.clone().padded())
                .map(TagType::Tag),
            just("see")
                .ignore_then(comment.clone().padded())
                .map(TagType::See),
            just("usage").ignore_then(
                choice((
                    just("[[").to(TagType::UsageStart),
                    just("]]").to(TagType::UsageEnd),
                    just('`')
                        .ignore_then(filter(|c| *c != '`').repeated())
                        .then_ignore(just('`'))
                        .collect()
                        .map(TagType::Usage),
                ))
                .padded(),
            ),
            just("export")
                .then(whitespace())
                .ignore_then(ident())
                .then_ignore(take_until(end()))
                .map(TagType::Export),
        )));

        choice((
            triple.ignore_then(choice((
                tag,
                variant,
                newline().to(TagType::Comment(String::new())),
                comment.map(TagType::Comment),
            ))),
            local.ignore_then(choice((
                func.clone().ignore_then(ident()).map(|name| TagType::Func {
                    name,
                    prefix: None,
                    kind: Kind::Local,
                }),
                ident()
                    .padded()
                    .then_ignore(just('='))
                    .map(|name| TagType::Expr {
                        name,
                        prefix: None,
                        kind: Kind::Local,
                    }),
            ))),
            func.clone()
                .ignore_then(dotted)
                .map(|(prefix, kind, name)| TagType::Func { prefix, name, kind }),
            choice((
                expr.clone()
                    .then_ignore(func)
                    .map(|(prefix, kind, name)| TagType::Func { prefix, name, kind }),
                expr.map(|(prefix, kind, name)| TagType::Expr { prefix, name, kind }),
            )),
            keyword("return")
                .ignore_then(name)
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
