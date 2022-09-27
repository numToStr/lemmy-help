use chumsky::{select, Parser};

use crate::{
    lexer::{Kind, TagType},
    parser::{impl_parse, Prefix, See},
};

use super::Usage;

#[derive(Debug, Clone)]
pub struct Type {
    pub prefix: Prefix,
    pub name: String,
    pub desc: (Vec<String>, Option<String>),
    pub kind: Kind,
    pub ty: String,
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
    .then(select! { TagType::Expr { prefix, name, kind } => (prefix, name, kind) })
    .map(
        |((((extract, (ty, desc)), see), usage), (prefix, name, kind))| Self {
            desc: (extract, desc),
            prefix: Prefix {
                left: prefix.clone(),
                right: prefix,
            },
            name,
            kind,
            ty,
            see,
            usage,
        },
    )
});

impl Type {
    pub fn rename_tag(&mut self, tag: String) {
        self.prefix.right = Some(tag);
    }

    pub fn is_public(&self, export: &str) -> bool {
        self.kind != Kind::Local && self.prefix.left.as_deref() == Some(export)
    }
}
