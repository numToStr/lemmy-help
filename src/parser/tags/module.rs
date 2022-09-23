use std::fmt::Display;

use chumsky::select;

use crate::{
    lexer::TagType,
    parser::{impl_parse, Divider},
};

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub desc: Option<String>,
}

impl_parse!(Module, {
    select! { TagType::Module(name, desc) => Self { name, desc } }
});

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self.desc.as_deref().unwrap_or_default();

        Divider('=').fmt(f)?;
        writeln!(
            f,
            "{desc}{}",
            format_args!("{:>w$}", format!("*{}*", self.name), w = 80 - desc.len())
        )
    }
}
