use chumsky::{select, Parser};

use crate::{lexer::TagType, parser::impl_parse, Accept, Visitor};

#[derive(Debug, Clone)]
pub struct See {
    pub refs: Vec<String>,
}

impl_parse!(See, {
    select! { TagType::See(x) => x }
        .repeated()
        .map(|refs| Self { refs })
});

impl<T: Visitor> Accept<T> for See {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.see(self, s)
    }
}
