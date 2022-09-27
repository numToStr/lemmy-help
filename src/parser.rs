mod node;
pub use node::*;
mod tags;
pub use tags::*;

macro_rules! impl_parse {
    ($id: ident, $ret: ty, $body: expr) => {
        impl $id {
            pub fn parse() -> impl chumsky::Parser<
                $crate::lexer::TagType,
                $ret,
                Error = chumsky::prelude::Simple<$crate::lexer::TagType>,
            > {
                $body
            }
        }
    };
    ($id: ident, $body: expr) => {
        crate::parser::impl_parse!($id, Self, $body);
    };
}

pub(super) use impl_parse;
