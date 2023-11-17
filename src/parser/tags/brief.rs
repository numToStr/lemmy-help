use chumsky::{primitive::just, select, IterParser, Parser};

use crate::{
    lexer::Token,
    parser::{LemmyParser, Node},
    Accept, Visitor,
};

#[derive(Debug, Clone)]
pub struct Brief<'src> {
    pub desc: Vec<&'src str>,
}

pub fn brief_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Node<'src>> {
    select! { Token::Comment(x) => x }
        .repeated()
        .collect()
        .delimited_by(just(Token::BriefStart), just(Token::BriefEnd))
        .map(|desc| Node::Brief(Brief { desc }))
}

impl<'src, T: Visitor> Accept<T> for Brief<'src> {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.brief(self, s)
    }
}
