mod lexer;
pub use lexer::*;

mod common;
pub use common::*;

mod brief;
pub use brief::*;
mod tag;
pub use tag::*;
mod class;
pub use class::*;
mod func;
pub use func::*;

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
    Func(Func),
    Class(Class),
    Tag(Tag),
    // Alias(Alias),
    // See(See),
    // Comment(Comment)
}

impl_parse!(Node, {
    choice((
        Brief::parse().map(Self::Brief),
        Tag::parse().map(Self::Tag),
        Func::parse().map(Self::Func),
        Class::parse().map(Self::Class),
        // See::parse().map(Self::See),
        // Alias::parse().map(Self::Alias),
        // Comment::parse().map(Self::Comment),
    ))
});

#[derive(Debug)]
pub struct LemmyHelp {
    pub nodes: Vec<Node>,
}

impl LemmyHelp {
    pub fn parse(src: &str) -> Result<Self, Vec<Simple<TagType>>> {
        let tokens = Lexer::parse().parse(src).unwrap();
        let stream = Stream::from_iter(src.len()..src.len() + 1, tokens.into_iter());

        Node::parse()
            .repeated()
            .parse(stream)
            .map(|nodes| Self { nodes })
    }
}
