use chumsky::{select, IterParser, Parser};

use crate::{
    lexer::{Op, Token, Ty},
    parser::{LemmyParser, Node, Prefix, See},
    Accept, Visitor,
};

use super::{see_parser, usage_parser, Usage};

#[derive(Debug, Clone)]
pub struct Type<'src> {
    pub desc: (Vec<&'src str>, Option<&'src str>),
    pub prefix: Prefix<'src>,
    pub op: Vec<Op<'src>>,
    pub ty: Ty<'src>,
    pub see: See<'src>,
    pub usage: Option<Usage<'src>>,
}

pub fn type_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Node<'src>> {
    select! { Token::Comment(x) => x }
        .repeated()
        .collect()
        .then(select! { Token::Type(ty,desc) => (ty,desc) })
        .then(see_parser())
        .then(usage_parser().or_not())
        .then(select! { Token::Expr(prefix,op) => (prefix,op) })
        .map(|((((extract, (ty, desc)), see), usage), (prefix, op))| {
            Node::Type(Type {
                desc: (extract, desc),
                prefix: Prefix {
                    left: Some(prefix),
                    right: Some(prefix),
                },
                op,
                ty,
                see,
                usage,
            })
        })
}

impl<'src, T: Visitor> Accept<T> for Type<'src> {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.r#type(self, s)
    }
}
