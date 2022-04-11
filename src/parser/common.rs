use chumsky::{select, Parser};

use crate::CommentType;

// Little helper macro for making parse function
#[macro_export]
macro_rules! impl_parse2 {
    ($id: ident, $body: expr) => {
        impl $id {
            pub fn parse() -> impl chumsky::Parser<
                crate::CommentType,
                Self,
                Error = chumsky::prelude::Simple<crate::CommentType>,
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
pub struct Str(pub String);

impl_parse2!(Str, select! {CommentType::Str(x) => x}.map(Self));
