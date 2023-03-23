use chumsky::{select, IterParser, Parser};

use crate::{lexer::Token, parser::LemmyParser, Accept, Visitor};

#[derive(Debug, Clone)]
pub struct See<'src> {
    pub refs: Vec<&'src str>,
}

pub fn see_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, See<'src>> {
    select! { Token::See(x) => x }
        .repeated()
        .collect()
        .map(|refs| See { refs })
}

impl<'src, T: Visitor> Accept<T> for See<'src> {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.see(self, s)
    }
}
