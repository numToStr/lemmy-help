use chumsky::{select, Parser};

use crate::{
    lexer::{Name, Op, TagType, Ty},
    parser::{impl_parse, Prefix, See},
    Accept, Visitor,
};

use super::Usage;

#[derive(Debug, Clone)]
pub struct Param {
    pub name: Name,
    pub ty: Ty,
    pub desc: Vec<String>,
}

impl_parse!(Param, {
    select! {
        TagType::Param(name, ty, desc) => (name, ty, desc)
    }
    .then(select! { TagType::Comment(x) => x }.repeated())
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
        Self { name, ty, desc }
    })
});

#[derive(Debug, Clone)]
pub struct Return {
    pub ty: Ty,
    pub name: Option<String>,
    pub desc: Vec<String>,
}

impl_parse!(Return, {
    select! {
        TagType::Return(ty, name, desc) => (ty, name, desc)
    }
    .then(select! { TagType::Comment(x) => x }.repeated())
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

        Self { name, ty, desc }
    })
});

#[derive(Debug, Clone)]
pub struct Func {
    pub op: Op,
    pub prefix: Prefix,
    pub desc: Vec<String>,
    pub params: Vec<Param>,
    pub returns: Vec<Return>,
    pub see: See,
    pub usage: Option<Usage>,
}

impl_parse!(Func, {
    select! {
        TagType::Comment(x) => x,
    }
    .repeated()
    .then(Param::parse().repeated())
    .then(Return::parse().repeated())
    .then(See::parse())
    .then(Usage::parse().or_not())
    .then(select! { TagType::Func(prefix, op) => (prefix, op) })
    .map(
        |(((((desc, params), returns), see), usage), (prefix, op))| Self {
            op,
            prefix: Prefix {
                left: Some(prefix.clone()),
                right: Some(prefix),
            },
            desc,
            params,
            returns,
            see,
            usage,
        },
    )
});

impl<T: Visitor> Accept<T> for Func {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.func(self, s)
    }
}
