use chumsky::{select, Parser};

use crate::{impl_parse, Comment, Object, TagType};

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub desc: Vec<Comment>,
    pub params: Vec<Object>,
    pub returns: Vec<Object>,
}

impl_parse!(Func, {
    Comment::parse()
        .repeated()
        .then(select! { TagType::Param(x) => x }.repeated())
        .then(select! { TagType::Return(x) => x }.repeated())
        .then(select! { TagType::Name(x) => x })
        .map(|(((desc, params), returns), name)| Self {
            name,
            desc,
            params,
            returns,
        })
});
