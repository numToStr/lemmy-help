use chumsky::{select, Parser};

use crate::{
    lexer::{Kind, TagType, Ty, TypeVal},
    parser::{impl_parse, Prefix, See},
};

use super::Usage;

#[derive(Debug, Clone)]
pub struct Param(pub TypeVal, pub Vec<String>);

impl_parse!(Param, {
    select! {
        TagType::Param(typeval, desc) => (typeval, desc)
    }
    .then(select! { TagType::Comment(x) => x }.repeated())
    .map(|((typeval, desc), extra)| {
        let desc = match desc {
            Some(d) => Vec::from([d])
                .into_iter()
                .chain(extra.into_iter())
                .collect(),
            None => extra,
        };
        Self(typeval, desc)
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
        TagType::Return { ty, name, desc } => (ty, name, desc)
    }
    .then(select! { TagType::Comment(x) => x }.repeated())
    .map(|((ty, name, desc), extra)| {
        let desc = match desc {
            Some(d) => Vec::from([d])
                .into_iter()
                .chain(extra.into_iter())
                .collect(),
            None => extra,
        };

        Self { name, ty, desc }
    })
});

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub kind: Kind,
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
    .then(select! {
        TagType::Func { prefix, name, kind } => (prefix, name, kind),
        TagType::Expr { prefix, name, kind } => (prefix, name, kind),
    })
    .map(
        |(((((desc, params), returns), see), usage), (prefix, name, kind))| Self {
            name,
            kind,
            prefix: Prefix {
                left: prefix.clone(),
                right: prefix,
            },
            desc,
            params,
            returns,
            see,
            usage,
        },
    )
});

impl Func {
    pub fn rename_tag(&mut self, tag: String) {
        self.prefix.right = Some(tag);
    }

    pub fn is_public(&self, export: &str) -> bool {
        self.kind != Kind::Local && self.prefix.left.as_deref() == Some(export)
    }
}
