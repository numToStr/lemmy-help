use std::fmt::Display;

use crate::parser::See;

use super::description;

#[derive(Debug)]
pub struct SeeDoc<'a>(pub &'a See);

impl Display for SeeDoc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        description!(f, "See: ~")?;
        for s in &self.0.refs {
            writeln!(f, "        |{s}|")?;
        }
        Ok(())
    }
}
