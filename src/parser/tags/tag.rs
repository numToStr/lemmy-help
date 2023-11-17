use chumsky::select;

use crate::{
    lexer::Token,
    parser::{LemmyParser, Node},
    Accept, Visitor,
};

#[derive(Debug, Clone)]
pub struct Tag<'src>(pub &'src str);

pub fn tag_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Node<'src>> {
    select! {
        Token::Tag(x) => Node::Tag(Tag(x))
    }
}

impl<'src, T: Visitor> Accept<T> for Tag<'src> {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.tag(self, s)
    }
}
