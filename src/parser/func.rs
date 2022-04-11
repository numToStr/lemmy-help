use chumsky::{select, Parser};

use crate::{impl_parse, CommentType, Object, Str};

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub desc: Option<Str>,
    pub params: Vec<Object>,
    pub returns: Vec<Object>,
}

impl_parse!(Func, {
    select! { CommentType::Func(x) => x }
        .then(Str::parse().or_not())
        .then(select! {CommentType::Param(x) => x}.repeated())
        .then(select! {CommentType::Return(x) => x}.repeated())
        .map(|(((name, desc), params), returns)| Self {
            name,
            desc,
            params,
            returns,
        })
});
