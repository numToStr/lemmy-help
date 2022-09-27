use std::fmt::Display;

use crate::parser::Brief;

#[derive(Debug)]
pub struct BriefDoc<'a>(pub &'a Brief);

impl Display for BriefDoc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0.desc.join("\n"))
    }
}
