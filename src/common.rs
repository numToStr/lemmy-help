use chumsky::{prelude::take_until, text, Parser};

// Little helper macro for making parse function
#[macro_export]
macro_rules! impl_parse {
    ($id: ident, $ret: ty, $body: expr) => {
        impl $id {
            pub fn parse(
            ) -> impl chumsky::Parser<char, $ret, Error = chumsky::prelude::Simple<char>> {
                $body
            }
        }
    };
    ($id: ident, $body: expr) => {
        crate::impl_parse!($id, Self, $body);
    };
}

// A type could be
// - primary = string|number|boolean
// - fn = func(...):string
// - enum = "one"|"two"|"three"
// - or: primary(|or)+
// - optional = primary?
// - table = table<string, string>
// - array = primary[]

#[derive(Debug)]
pub struct Desc(pub String);

impl_parse!(Desc, {
    take_until(text::newline()).map(|(x, _)| Self(x.into_iter().collect::<String>()))
});
