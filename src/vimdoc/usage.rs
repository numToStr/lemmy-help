use std::fmt::Display;

use crate::parser::Usage;

use super::description;

#[derive(Debug)]
pub struct UsageDoc<'a>(pub &'a Usage);

impl Display for UsageDoc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        description!(f, "Usage: ~")?;
        writeln!(f, "{:>9}", ">")?;
        writeln!(f, "{}", textwrap::indent(&self.0.code, "            "))?;
        writeln!(f, "{:>9}", "<")
    }
}
