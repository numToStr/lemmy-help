use chumsky::select;

use crate::{
    lexer::Token,
    parser::{LemmyParser, Node},
    Accept, Visitor,
};

#[derive(Debug, Clone)]
pub struct Module<'src> {
    pub name: &'src str,
    pub desc: Option<&'src str>,
}

pub fn mod_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Node<'src>> {
    select! {
        Token::Module(name,desc) => Node::Module(Module { name,desc })
    }
}

impl<'src, T: Visitor> Accept<T> for Module<'src> {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.module(self, s)
    }
}
