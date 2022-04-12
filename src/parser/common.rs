use std::fmt::Display;

use chumsky::select;

use crate::TagType;

// Little helper macro for making parse function
#[macro_export]
macro_rules! impl_parse {
    ($id: ident, $body: expr) => {
        impl $id {
            pub fn parse() -> impl chumsky::Parser<
                crate::TagType,
                Self,
                Error = chumsky::prelude::Simple<crate::TagType>,
            > {
                $body
            }
        }
    };
}

// A TYPE could be
// - primary = string|number|boolean
// - fn = func(...):string
// - enum = "one"|"two"|"three"
// - or: primary (| primary)+
// - optional = primary?
// - table = table<string, string>
// - array = primary[]

/// ---@comment
#[derive(Debug)]
pub struct Comment(pub String);

impl_parse!(Comment, select! { TagType::Comment(x) => Self(x)});

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// some utility functions

pub fn asterisk(s: &str) -> String {
    format!("*{}*", s)
}

pub fn create_title(name: &str, tag: &str) -> String {
    let t = asterisk(tag);

    format!("{}{}{}", name, " ".repeat(80 - (name.len() + t.len())), t)
}
