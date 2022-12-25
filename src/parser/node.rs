use chumsky::{
    prelude::{any, choice, Simple},
    select, Parser, Stream,
};

use crate::{
    lexer::{Lexer, TagType},
    parser::{Alias, Brief, Class, Divider, Func, Module, Tag, Type},
    Accept, Visitor,
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

impl<T: Visitor> Accept<T> for Node {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        match self {
            Self::Brief(x) => x.accept(n, s),
            Self::Tag(x) => x.accept(n, s),
            Self::Alias(x) => x.accept(n, s),
            Self::Func(x) => x.accept(n, s),
            Self::Class(x) => x.accept(n, s),
            Self::Type(x) => x.accept(n, s),
            Self::Module(x) => x.accept(n, s),
            Self::Divider(x) => x.accept(n, s),
            _ => unimplemented!(),
        }
    }
}

impl Node {
    fn init() -> impl Parser<TagType, Vec<Node>, Error = Simple<TagType>> {
        Node::parse().repeated().flatten()
    }

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
        let tokens = Lexer::init().parse(src).unwrap();
        let stream = Stream::from_iter(src.len()..src.len() + 1, tokens.into_iter());

        Node::init().parse(stream)
    }
}
