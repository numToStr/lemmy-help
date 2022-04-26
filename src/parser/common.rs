use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Prefix {
    pub left: Option<String>,
    pub right: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scope {
    Dot,
    Colon,
    Local,
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dot => f.write_str("."),
            Self::Colon => f.write_str(":"),
            Self::Local => f.write_str("#PRIVATE#"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Usage {
    pub code: String,
}

impl std::fmt::Display for Usage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::description!(f, "Usage: ~")?;
        writeln!(f, "{:>9}", ">")?;
        writeln!(f, "{:>w$}", self.code, w = 12 + self.code.len())?;
        writeln!(f, "{:>9}", "<")
    }
}
