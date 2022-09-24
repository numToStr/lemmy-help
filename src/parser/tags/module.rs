use chumsky::select;

use crate::{lexer::TagType, parser::impl_parse};

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub desc: Option<String>,
}

impl_parse!(Module, {
    select! { TagType::Module(name, desc) => Self { name, desc } }
});
