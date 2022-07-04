use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{parser, TagType};

#[derive(Debug, Default, Clone)]
pub struct Prefix {
    pub left: Option<String>,
    pub right: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Kind {
    Dot,
    Colon,
    Local,
}

impl Display for Kind {
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

parser!(Usage, {
    select! { TagType::Usage(code) => Self { code } }
});

impl std::fmt::Display for Usage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::description!(f, "Usage: ~")?;
        writeln!(f, "{:>9}", ">")?;
        writeln!(f, "{:>w$}", self.code, w = 12 + self.code.len())?;
        writeln!(f, "{:>9}", "<")
    }
}

#[derive(Debug, Clone)]
pub struct See {
    pub refs: Vec<String>,
}

parser!(See, {
    select! { TagType::See(x) => x }
        .repeated()
        .map(|refs| Self { refs })
});

impl std::fmt::Display for See {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::description!(f, "See: ~")?;
        for s in &self.refs {
            writeln!(f, "        |{}|", s)?;
        }
        Ok(())
    }
}
