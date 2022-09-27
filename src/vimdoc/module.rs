use std::fmt::Display;

use crate::parser::{Divider, Module};

use super::divider::DividerDoc;

#[derive(Debug)]
pub struct ModuleDoc<'a>(pub &'a Module);

impl Display for ModuleDoc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self.0.desc.as_deref().unwrap_or_default();

        DividerDoc(&Divider('=')).fmt(f)?;
        writeln!(
            f,
            "{desc}{}",
            format_args!("{:>w$}", format!("*{}*", self.0.name), w = 80 - desc.len())
        )
    }
}
