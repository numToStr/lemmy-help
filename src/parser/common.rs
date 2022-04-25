use std::fmt::Display;

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
pub struct Prefix {
    pub left: String,
    pub right: String,
}
