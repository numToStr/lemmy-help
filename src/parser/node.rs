use std::fmt::Display;

use chumsky::{
    prelude::{any, choice, Simple},
    select, Parser, Stream,
};

use crate::{
    lexer::{Lexer, TagType},
    parser::{Alias, Brief, Class, Divider, Func, Module, Tag, Type},
};

use super::impl_parse;

#[derive(Debug, Clone)]
pub enum Node {
    Module(Module),
    Divider(Divider),
    Brief(Brief),
    Tag(Tag),
    Func(Func),
    Class(Class),
    Alias(Alias),
    Type(Type),
    Export(String),
    Toc(String),
}

impl_parse!(Node, Option<Self>, {
    choice((
        Module::parse().map(Self::Module),
        Divider::parse().map(Self::Divider),
        Brief::parse().map(Self::Brief),
        Tag::parse().map(Self::Tag),
        Func::parse().map(Self::Func),
        Class::parse().map(Self::Class),
        Alias::parse().map(Self::Alias),
        Type::parse().map(Self::Type),
        select! {
            TagType::Export(x) => Self::Export(x),
            TagType::Toc(x) => Self::Toc(x),
        },
    ))
    .map(Some)
    // Skip useless nodes
    .or(any().to(None))
});

impl Node {
    /// Creates stream of AST nodes from emmylua
    ///
    /// ```
    /// let src = r#"
    /// local U = {}
    ///
    /// ---Add two integar and print it
    /// ---@param this number First number
    /// ---@param that number Second number
    /// function U.sum(this, that)
    ///     print(this + that)
    /// end
    ///
    /// return U
    /// "#;
    ///
    /// let nodes = lemmy_help::parser::Node::new(src).unwrap();
    /// assert!(!nodes.is_empty());
    /// ```
    pub fn new(src: &str) -> Result<Vec<Node>, Vec<Simple<TagType>>> {
        let tokens = Lexer::parse(src).unwrap();
        let stream = Stream::from_iter(src.len()..src.len() + 1, tokens.into_iter());

        Node::parse().repeated().flatten().parse(stream)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Brief(x) => x.fmt(f),
            Self::Tag(x) => x.fmt(f),
            Self::Func(x) => x.fmt(f),
            Self::Class(x) => x.fmt(f),
            Self::Alias(x) => x.fmt(f),
            Self::Type(x) => x.fmt(f),
            Self::Module(x) => x.fmt(f),
            Self::Divider(x) => x.fmt(f),
            _ => unimplemented!(),
        }
    }
}
