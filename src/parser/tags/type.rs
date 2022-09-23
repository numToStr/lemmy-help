use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{
    lexer::{Kind, TagType},
    parser::{description, header, impl_parse, Prefix, See},
};

use super::Usage;

#[derive(Debug, Clone)]
pub struct Type {
    pub desc: Vec<String>,
    pub prefix: Prefix,
    pub name: String,
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
    .then(select! { TagType::Type(ty) => ty })
    .then(See::parse())
    .then(Usage::parse().or_not())
    .then(select! { TagType::Expr { prefix, name, kind } => (prefix, name, kind) })
    .map(|((((desc, ty), see), usage), (prefix, name, kind))| Self {
        desc,
        prefix: Prefix {
            left: prefix.clone(),
            right: prefix,
        },
        name,
        kind,
        ty,
        see,
        usage,
    })
});

impl Type {
    pub fn rename_tag(&mut self, tag: String) {
        self.prefix.right = Some(tag);
    }

    pub fn is_public(&self, export: &str) -> bool {
        self.kind != Kind::Local && self.prefix.left.as_deref() == Some(export)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        header!(
            f,
            &format!(
                "{}{}{}",
                self.prefix.left.as_deref().unwrap_or_default(),
                self.kind.as_char(),
                self.name
            ),
            &format!(
                "{}{}{}",
                self.prefix.right.as_deref().unwrap_or_default(),
                self.kind.as_char(),
                self.name
            )
        )?;

        if !self.desc.is_empty() {
            description!(f, &self.desc.join("\n"))?;
        }

        writeln!(f)?;

        description!(f, "Type: ~")?;
        f.write_fmt(format_args!(
            "{:>w$}\n\n",
            format!("({})", self.ty),
            w = 10 + self.ty.len()
        ))?;

        if !self.see.refs.is_empty() {
            writeln!(f, "{}", self.see)?;
        }

        if let Some(usage) = &self.usage {
            writeln!(f, "{usage}")?;
        }

        Ok(())
    }
}
