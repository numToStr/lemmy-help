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
