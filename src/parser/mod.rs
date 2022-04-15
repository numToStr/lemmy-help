mod emmy;
pub use emmy::*;

mod common;
pub use common::*;

mod brief;
pub use brief::*;
mod alias;
pub use alias::*;
mod tag;
pub use tag::*;
mod class;
pub use class::*;
mod func;
pub use func::*;

use std::fmt::Display;

use chumsky::{
    prelude::{choice, Simple},
    Parser, Stream,
};

use crate::impl_parse;

// ---@tag @comment

// ---@field [public|protected|private] field_name FIELD_TYPE[|OTHER_TYPE] [@comment]

// ---@param param_name MY_TYPE[|other_type] [@comment]

// ---@type MY_TYPE[|OTHER_TYPE] [@comment]

// ---@alias NEW_NAME TYPE [@comment]

// ---@see @comment

// ---@return MY_TYPE[|OTHER_TYPE] [@comment]

#[derive(Debug)]
pub enum Node {
    Brief(Brief),
    Tag(Tag),
    Func(Func),
    Class(Class),
    Alias(Alias),
    // See(See),
    // Comment(Comment)
}

impl_parse!(Node, {
    choice((
        Brief::parse().map(Self::Brief),
        Tag::parse().map(Self::Tag),
        Func::parse().map(Self::Func),
        Class::parse().map(Self::Class),
        Alias::parse().map(Self::Alias),
        // See::parse().map(Self::See),
        // Comment::parse().map(Self::Comment),
    ))
});

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Brief(x) => x.fmt(f),
            Self::Tag(x) => x.fmt(f),
            Self::Func(x) => x.fmt(f),
            Self::Class(x) => x.fmt(f),
            Self::Alias(x) => x.fmt(f),
            // _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct LemmyHelp {
    pub nodes: Vec<Node>,
}

impl LemmyHelp {
    pub fn parse(src: &str) -> Result<Self, Vec<Simple<TagType>>> {
        let tokens = Emmy::parse().parse(src).unwrap();
        let stream = Stream::from_iter(src.len()..src.len() + 1, tokens.into_iter());

        Node::parse()
            .repeated()
            .parse(stream)
            .map(|nodes| Self { nodes })
    }
}

impl Display for LemmyHelp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ele in &self.nodes {
            writeln!(f, "{}", ele)?;
        }

        write!(f, "")
    }
}
