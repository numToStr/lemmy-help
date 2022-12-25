use chumsky::select;

use crate::{lexer::TagType, parser::impl_parse, Accept, Visitor};

#[derive(Debug, Clone)]
pub struct Tag(pub String);

impl_parse!(Tag, {
    select! { TagType::Tag(x) => Self(x) }
});

impl<T: Visitor> Accept<T> for Tag {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.tag(self, s)
    }
}
