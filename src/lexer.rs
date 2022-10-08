mod token;
pub use token::*;

use std::ops::Range;

use chumsky::{
    prelude::{any, choice, end, filter, just, take_until, Simple},
    recursive::recursive,
    text::{ident, keyword, newline, whitespace, TextParser},
    Parser,
};

type Spanned = (TagType, Range<usize>);

const C: [char; 3] = ['.', '_', '-'];

#[derive(Debug)]
pub struct Lexer;

impl Lexer {
    /// Parse emmylua/lua files into rust token
    pub fn parse(src: &str) -> Result<Vec<Spanned>, Vec<Simple<char>>> {
        let triple = just("---");
        let space = just(' ').repeated().at_least(1);
        let till_eol = take_until(newline());

        let comment = till_eol.map(|(x, _)| x.iter().collect());
        let desc = space.ignore_then(comment).or_not();

        let scope = choice((
            keyword("public").to(Scope::Public),
            keyword("protected").to(Scope::Protected),
            keyword("private").to(Scope::Private),
        ));

        let private = just("private")
            .then_ignore(newline())
            .then_ignore(choice((
                // eat up all the emmylua, if any, then one valid token
                triple
                    .then(till_eol)
                    .padded()
                    .repeated()
                    .ignore_then(ident()),
                // if there is no emmylua, just eat the next token
                // so the next parser won't recognize the code
                ident().padded(),
            )))
            .ignored();

        let variant = just('|')
            .then_ignore(space)
            .ignore_then(
                just('\'')
                    .ignore_then(filter(|c| c != &'\'').repeated())
                    .then_ignore(just('\''))
                    .collect(),
            )
            .then(
                space
                    .ignore_then(just('#').ignore_then(space).ignore_then(comment))
                    .or_not(),
            )
            .map(|(t, d)| TagType::Variant(t, d));

        let name = Self::ty_name();

        let tag = just('@').ignore_then(choice((
            private.to(TagType::Skip),
            just("toc")
                .ignore_then(space)
                .ignore_then(comment)
                .map(TagType::Toc),
            just("mod")
                .then_ignore(space)
                .ignore_then(name.clone())
                .then(desc)
                .map(|(name, desc)| TagType::Module(name, desc)),
            just("divider")
                .ignore_then(space)
                .ignore_then(any())
                .map(TagType::Divider),
            just("brief").ignore_then(space).ignore_then(choice((
                just("[[").to(TagType::BriefStart),
                just("]]").to(TagType::BriefEnd),
            ))),
            just("param")
                .ignore_then(space)
                .ignore_then(ident().then(just('?').or_not().map(|x| x.is_some())))
                .then_ignore(space)
                .then(Self::ty())
                .then(desc)
                .map(|(((name, optional), ty), desc)| TagType::Param {
                    name,
                    optional,
                    ty,
                    desc,
                }),
            just("return")
                .ignore_then(space)
                .ignore_then(Self::ty())
                .then(choice((
                    newline().to((None, None)),
                    space.ignore_then(choice((
                        just('#').ignore_then(comment).map(|x| (None, Some(x))),
                        ident().then(desc).map(|(name, desc)| (Some(name), desc)),
                    ))),
                )))
                .map(|(ty, (name, desc))| TagType::Return { ty, name, desc }),
            just("class")
                .ignore_then(space)
                .ignore_then(name.clone())
                .map(TagType::Class),
            just("field")
                .ignore_then(space.ignore_then(scope).or_not())
                .then_ignore(space)
                .then(ident())
                .then_ignore(space)
                .then(Self::ty())
                .then(desc)
                .map(|(((scope, name), ty), desc)| TagType::Field {
                    scope: scope.unwrap_or(Scope::Public),
                    name,
                    ty,
                    desc,
                }),
            just("alias")
                .ignore_then(space)
                .ignore_then(name)
                .then(space.ignore_then(Self::ty()).or_not())
                .map(|(name, ty)| TagType::Alias(name, ty)),
            just("type")
                .ignore_then(space)
                .ignore_then(Self::ty())
                .then(desc)
                .map(|(ty, desc)| TagType::Type(ty, desc)),
            just("tag")
                .ignore_then(space)
                .ignore_then(comment)
                .map(TagType::Tag),
            just("see")
                .ignore_then(space)
                .ignore_then(comment)
                .map(TagType::See),
            just("usage").ignore_then(space).ignore_then(choice((
                just("[[").to(TagType::UsageStart),
                just("]]").to(TagType::UsageEnd),
                just('`')
                    .ignore_then(filter(|c| *c != '`').repeated())
                    .then_ignore(just('`'))
                    .collect()
                    .map(TagType::Usage),
            ))),
            just("export")
                .ignore_then(space)
                .ignore_then(ident())
                .then_ignore(take_until(end()))
                .map(TagType::Export),
        )));

        let local = keyword("local").padded();
        let func = keyword("function").padded();
        let assign = just('=').padded();

        let dotted = choice((
            ident()
                .then(choice((just('.').to(Kind::Dot), just(':').to(Kind::Colon))))
                .then(ident())
                .map(|((prefix, scope), name)| (Some(prefix), scope, name)),
            ident().map(|name| (None, Kind::Local, name)),
        ));

        let expr = dotted.clone().then_ignore(assign);

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
                ident().then_ignore(assign).map(|name| TagType::Expr {
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
                .ignore_then(ident().padded())
                .then_ignore(end())
                .map(TagType::Export),
            till_eol.to(TagType::Skip),
        ))
        .padded()
        .map_with_span(|t, r| (t, r))
        .repeated()
        .parse(src)
    }

    pub fn ty() -> impl Parser<char, Ty, Error = Simple<char>> {
        recursive(|inner| {
            let comma = just(',').padded();
            let colon = just(':').padded();

            let any = just("any").to(Ty::Any);
            let unknown = just("unknown").to(Ty::Unknown);
            let nil = just("nil").to(Ty::Nil);
            let boolean = just("boolean").to(Ty::Boolean);
            let string = just("string").to(Ty::String);
            let num = just("number").to(Ty::Number);
            let int = just("integer").to(Ty::Integer);
            let function = just("function").to(Ty::Function);
            let thread = just("thread").to(Ty::Thread);
            let userdata = just("userdata").to(Ty::Userdata);
            let lightuserdata = just("lightuserdata").to(Ty::Lightuserdata);

            #[inline]
            fn union_array(
                p: impl Parser<char, Ty, Error = Simple<char>>,
                inner: impl Parser<char, Ty, Error = Simple<char>>,
            ) -> impl Parser<char, Ty, Error = Simple<char>> {
                p.then(just("[]").repeated())
                    .foldl(|arr, _| Ty::Array(Box::new(arr)))
                    // NOTE: Not the way I wanted i.e., Ty::Union(Vec<Ty>) it to be, but it's better than nothing
                    .then(just('|').padded().ignore_then(inner).repeated())
                    .foldl(|x, y| Ty::Union(Box::new(x), Box::new(y)))
            }

            let list_like = ident()
                .padded()
                .then(just('?').or_not().map(|c| match c {
                    Some(_) => Kv::Opt as fn(_, _) -> _,
                    None => Kv::Req as fn(_, _) -> _,
                }))
                .then_ignore(colon)
                .then(inner.clone())
                .map(|((n, attr), t)| attr(n, t))
                .separated_by(comma)
                .allow_trailing();

            let fun = just("fun")
                .ignore_then(
                    list_like
                        .clone()
                        .delimited_by(just('(').then(whitespace()), whitespace().then(just(')'))),
                )
                .then(colon.ignore_then(inner.clone().map(Box::new)).or_not())
                .map(|(param, ret)| Ty::Fun(param, ret));

            let table = just("table")
                .ignore_then(
                    just('<')
                        .ignore_then(inner.clone().map(Box::new))
                        .then_ignore(comma)
                        .then(inner.clone().map(Box::new))
                        .then_ignore(just('>'))
                        .or_not(),
                )
                .map(Ty::Table);

            let dict = list_like
                .delimited_by(just('{').then(whitespace()), whitespace().then(just('}')))
                .map(Ty::Dict);

            let ty_name = Self::ty_name().map(Ty::Ref);

            let parens = inner
                .clone()
                .delimited_by(just('(').padded(), just(')').padded());

            choice((
                union_array(any, inner.clone()),
                union_array(unknown, inner.clone()),
                union_array(nil, inner.clone()),
                union_array(boolean, inner.clone()),
                union_array(string, inner.clone()),
                union_array(num, inner.clone()),
                union_array(int, inner.clone()),
                union_array(function, inner.clone()),
                union_array(thread, inner.clone()),
                union_array(userdata, inner.clone()),
                union_array(lightuserdata, inner.clone()),
                union_array(fun, inner.clone()),
                union_array(table, inner.clone()),
                union_array(dict, inner.clone()),
                union_array(parens, inner.clone()),
                union_array(ty_name, inner),
            ))
        })
    }

    #[inline]
    fn ty_name() -> impl Parser<char, String, Error = Simple<char>> + Clone {
        filter(|x: &char| x.is_alphanumeric() || C.contains(x))
            .repeated()
            .collect()
    }
}
