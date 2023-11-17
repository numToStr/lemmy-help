use chumsky::{prelude::choice, select, IterParser, Parser};

use crate::{
    lexer::{Member, Token, Ty},
    parser::{LemmyParser, Node, Prefix},
    Accept, Visitor,
};

#[derive(Debug, Clone)]
pub enum AliasKind<'src> {
    Type(Ty<'src>),
    Enum(Vec<(Member<'src>, Option<&'src str>)>),
}

#[derive(Debug, Clone)]
pub struct Alias<'src> {
    pub name: &'src str,
    pub desc: Vec<&'src str>,
    pub kind: AliasKind<'src>,
    pub prefix: Prefix<'src>,
}

pub fn alias_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Node<'src>> {
    select! { Token::Comment(x) => x }
        .repeated()
        .collect()
        .then(choice((
            select! { Token::Alias(name,Some(ty)) => (name,AliasKind::Type(ty)) },
            select! { Token::Alias(name, ..) => name }.then(
                select! { Token::Variant(ty,desc) => (ty,desc) }
                    .repeated()
                    .collect()
                    .map(AliasKind::Enum),
            ),
        )))
        .map(|(desc, (name, kind))| {
            Node::Alias(Alias {
                name,
                desc,
                kind,
                prefix: Prefix::default(),
            })
        })
}

impl<'src, T: Visitor> Accept<T> for Alias<'src> {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.alias(self, s)
    }
}
