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
    Str::parse()
        .or_not()
        .then(select! { CommentType::Param(x) => x }.repeated())
        .then(select! { CommentType::Return(x) => x }.repeated())
        .then(select! { CommentType::Name(x) => x })
        .map(|(((desc, params), returns), name)| Self {
            name,
            desc,
            params,
            returns,
        })
});
