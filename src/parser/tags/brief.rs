use std::fmt::Display;

use chumsky::{prelude::just, select, Parser};

use crate::{lexer::TagType, parser::impl_parse};

#[derive(Debug, Clone)]
pub struct Brief {
    pub desc: Vec<String>,
}

impl_parse!(Brief, {
    select! {
        TagType::Comment(x) => x,
    }
    .repeated()
    .delimited_by(just(TagType::BriefStart), just(TagType::BriefEnd))
    .map(|desc| Self { desc })
});

impl Display for Brief {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.desc.join("\n"))
    }
}
