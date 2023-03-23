use chumsky::{select, IterParser, Parser};

use crate::{
    lexer::{Name, Op, Token, Ty},
    parser::{LemmyParser, Node, Prefix, See},
    Accept, Visitor,
};

use super::{see_parser, usage_parser, Usage};

#[derive(Debug, Clone)]
pub struct Param<'src> {
    pub name: Name<'src>,
    pub ty: Ty<'src>,
    pub desc: Vec<&'src str>,
}

fn param_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Param<'src>> {
    select! { Token::Param(name,ty,desc) => (name,ty,desc) }
        .then(
            select! { Token::Comment(x) => x }
                .repeated()
                .collect::<Vec<&'src str>>(),
        )
        .map(|((name, ty, desc), extra)| {
            let desc = match desc {
                Some(d) => {
                    let mut new_desc = Vec::with_capacity(extra.len() + 1);
                    new_desc.push(d);
                    new_desc.extend(extra);
                    new_desc
                }
                None => extra,
            };
            Param { name, ty, desc }
        })
}

#[derive(Debug, Clone)]
pub struct Return<'src> {
    pub ty: Ty<'src>,
    pub name: Option<&'src str>,
    pub desc: Vec<&'src str>,
}

fn return_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Return<'src>> {
    select! { Token::Return(ty,name,desc) => (ty,name,desc) }
        .then(
            select! { Token::Comment(x) => x }
                .repeated()
                .collect::<Vec<&'src str>>(),
        )
        .map(|((ty, name, desc), extra)| {
            let desc = match desc {
                Some(d) => {
                    let mut new_desc = Vec::with_capacity(extra.len() + 1);
                    new_desc.push(d);
                    new_desc.extend(extra);
                    new_desc
                }
                None => extra,
            };
            Return { name, ty, desc }
        })
}

#[derive(Debug, Clone)]
pub struct Func<'src> {
    pub prefix: Prefix<'src>,
    pub op: Vec<Op<'src>>,
    pub desc: Vec<&'src str>,
    pub params: Vec<Param<'src>>,
    pub returns: Vec<Return<'src>>,
    pub see: See<'src>,
    pub usage: Option<Usage<'src>>,
}

pub fn func_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Node<'src>> {
    select! { Token::Comment(x) => x }
        .repeated()
        .collect()
        .then(param_parser().repeated().collect())
        .then(return_parser().repeated().collect())
        .then(see_parser())
        .then(usage_parser().or_not())
        .then(select! { Token::Func(prefix,op) => (prefix,op) })
        .map(
            |(((((desc, params), returns), see), usage), (prefix, op))| {
                Node::Func(Func {
                    prefix: Prefix {
                        left: Some(prefix),
                        right: Some(prefix),
                    },
                    op,
                    desc,
                    params,
                    returns,
                    see,
                    usage,
                })
            },
        )
}

impl<'src, T: Visitor> Accept<T> for Func<'src> {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.func(self, s)
    }
}
