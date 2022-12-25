use chumsky::select;

use crate::{lexer::TagType, parser::impl_parse, Accept, Visitor};

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub desc: Option<String>,
}

impl_parse!(Module, {
    select! { TagType::Module(name, desc) => Self { name, desc } }
});

impl<T: Visitor> Accept<T> for Module {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.module(self, s)
    }
}
