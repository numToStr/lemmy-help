use std::fmt::Display;

use crate::parser::Divider;

#[derive(Debug)]
pub struct DividerDoc<'a>(pub &'a Divider);

impl Display for DividerDoc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", (self.0).0.to_string().repeat(80))
    }
}
