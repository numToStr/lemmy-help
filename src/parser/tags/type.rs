use chumsky::{select, Parser};

use crate::{
    lexer::{Op, TagType, Ty},
    parser::{impl_parse, Prefix, See},
};

use super::Usage;

#[derive(Debug, Clone)]
pub struct Type {
    pub desc: (Vec<String>, Option<String>),
    pub op: Op,
    pub prefix: Prefix,
    pub ty: Ty,
    pub see: See,
    pub usage: Option<Usage>,
}

impl_parse!(Type, {
    select! {
        TagType::Comment(x) => x
    }
    .repeated()
    .then(select! { TagType::Type(ty, desc) => (ty, desc) })
    .then(See::parse())
    .then(Usage::parse().or_not())
    .then(select! { TagType::Expr(prefix, op) => (prefix, op) })
    .map(
        |((((extract, (ty, desc)), see), usage), (prefix, op))| Self {
            desc: (extract, desc),
            prefix: Prefix {
                left: Some(prefix.to_owned()),
                right: Some(prefix),
            },
            op,
            ty,
            see,
            usage,
        },
    )
});
