use std::fmt::Display;

use chumsky::{
    prelude::{any, choice},
    select, Parser,
};

use crate::{parser, Alias, Brief, Class, Divider, Func, Module, Tag, TagType, Type};

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
}

parser!(Node, Option<Self>, {
    choice((
        Module::parse().map(Self::Module),
        Divider::parse().map(Self::Divider),
        Brief::parse().map(Self::Brief),
        Tag::parse().map(Self::Tag),
        Func::parse().map(Self::Func),
        Class::parse().map(Self::Class),
        Alias::parse().map(Self::Alias),
        Type::parse().map(Self::Type),
        // We need this to match exported types/funcs etc.
        select! { TagType::Export(x) => Self::Export(x) },
    ))
    .map(Some)
    // This will skip useless nodes
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
            Self::Module(x) => x.fmt(f),
            Self::Divider(x) => x.fmt(f),
            _ => unimplemented!(),
        }
    }
}
