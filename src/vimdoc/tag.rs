use std::fmt::Display;

use crate::parser::Tag;

#[derive(Debug)]
pub struct TagDoc<'a>(pub &'a Tag);

impl Display for TagDoc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>80}", format!("*{}*", (self.0).0))
    }
}
