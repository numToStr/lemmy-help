use chumsky::select;

use crate::{
    lexer::Token,
    parser::{LemmyParser, Node},
    Accept, Visitor,
};

#[derive(Debug, Clone)]
pub struct Divider(pub char);

pub fn divider_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Node<'src>> {
    select! {
        Token::Divider(rune) => Node::Divider(Divider(rune))
    }
}

impl<T: Visitor> Accept<T> for Divider {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.divider(self, s)
    }
}
