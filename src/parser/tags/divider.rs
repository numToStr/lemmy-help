use chumsky::select;

use crate::{lexer::TagType, parser::impl_parse, Accept, Visitor};

#[derive(Debug, Clone)]
pub struct Divider(pub char);

impl_parse!(Divider, {
    select! { TagType::Divider(rune) => Self(rune) }
});

impl<T: Visitor> Accept<T> for Divider {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.divider(self, s)
    }
}
