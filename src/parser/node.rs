use chumsky::{
    prelude::choice,
    primitive::{any, end},
    recovery::skip_then_retry_until,
    select, Parser,
};

use crate::{
    parser::{Alias, Brief, Class, Divider, Func, Module, Token, Type},
    Accept, Visitor,
};

use super::{
    alias_parser, brief_parser, class_parser, divider_parser, func_parser, mod_parser, tag_parser,
    type_parser, LemmyParser, Tag,
};

#[derive(Debug, Clone)]
pub enum Node<'src> {
    Module(Module<'src>),
    Divider(Divider),
    Brief(Brief<'src>),
    Tag(Tag<'src>),
    Func(Func<'src>),
    Class(Class<'src>),
    Alias(Alias<'src>),
    Type(Type<'src>),
    Export(&'src str),
    Toc(&'src str),
}

pub fn node_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Node<'src>> {
    choice((
        mod_parser(),
        divider_parser(),
        brief_parser(),
        tag_parser(),
        func_parser(),
        class_parser(),
        alias_parser(),
        type_parser(),
        select! {
            Token::Export(x) => Node::Export(x),
            Token::Toc(x) => Node::Toc(x),
        },
    ))
    .recover_with(skip_then_retry_until(any().ignored(), end()))
}

impl<'src, T: Visitor> Accept<T> for Node<'src> {
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
