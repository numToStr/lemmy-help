mod emmy;
pub use emmy::*;

mod tags;
pub use tags::*;

use std::fmt::Display;

use chumsky::{
    prelude::{any, choice, Simple},
    select, Parser, Stream,
};

// Little helper macro for making parse function
#[macro_export]
macro_rules! impl_parse {
    ($id: ident, $ret: ty, $body: expr) => {
        impl $id {
            pub fn parse() -> impl chumsky::Parser<
                crate::TagType,
                $ret,
                Error = chumsky::prelude::Simple<crate::TagType>,
            > {
                $body
            }
        }
    };
    ($id: ident, $body: expr) => {
        crate::impl_parse!($id, Self, $body);
    };
}

// A TYPE could be
// - primary = string|number|boolean
// - fn = func(...):string
// - enum = "one"|"two"|"three"
// - or: primary (| primary)+
// - optional = primary?
// - table = table<string, string>
// - array = primary[]

// ---@tag @comment

// ---@field [public|protected|private] field_name FIELD_TYPE[|OTHER_TYPE] [@comment]

// ---@param param_name MY_TYPE[|other_type] [@comment]

// ---@type MY_TYPE[|OTHER_TYPE] [@comment]

// ---@alias NEW_NAME TYPE [@comment]

// ---@see @comment

// ---@return MY_TYPE[|OTHER_TYPE] [@comment]

#[derive(Debug, Clone)]
pub enum Node {
    Brief(Brief),
    Tag(Tag),
    Func(Func),
    Class(Class),
    Alias(Alias),
    Type(Type),
    Export(String),
    // See(See),
    // Comment(Comment)
}

impl_parse!(Node, Option<Self>, {
    choice((
        Brief::parse().map(Self::Brief),
        Tag::parse().map(Self::Tag),
        Func::parse().map(Self::Func),
        Class::parse().map(Self::Class),
        Alias::parse().map(Self::Alias),
        Type::parse().map(Self::Type),
        // We need this export to properly create the docs
        // Like there is not point in creating docs for internal things
        // NOTE: This is inserted by the lua parser
        select! { TagType::Export(x) => Self::Export(x) },
    ))
    .map(Some)
    // This will skip extra nodes which were probably injected by the fronted parsers
    // i.e. ---@func | ---@expr
    .or(any().to(None))
});

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Brief(x) => x.fmt(f),
            Self::Tag(x) => x.fmt(f),
            Self::Func(x) => x.fmt(f),
            Self::Class(x) => x.fmt(f),
            Self::Alias(x) => x.fmt(f),
            Self::Type(x) => x.fmt(f),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct LemmyHelp {
    pub nodes: Vec<Node>,
}

impl LemmyHelp {
    pub fn parse(src: &str) -> Result<Self, Vec<Simple<TagType>>> {
        let tokens = Emmy::parse(src).unwrap();
        let stream = Stream::from_iter(src.len()..src.len() + 1, tokens.into_iter());

        Node::parse()
            .repeated()
            .flatten()
            .parse(stream)
            .map(|nodes| Self { nodes })
    }
}

impl Display for LemmyHelp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(Node::Export(export)) = &self.nodes.last() {
            for ele in &self.nodes {
                match ele {
                    Node::Export(..) => {}
                    Node::Func(Func { name, .. }) | Node::Type(Type { name, .. }) => {
                        if let Name::Member(member, ..) = name {
                            if member == export {
                                writeln!(f, "{}", ele)?;
                            }
                        }
                    }
                    _ => writeln!(f, "{}", ele)?,
                }
            }
        }

        write!(f, "")
    }
}
