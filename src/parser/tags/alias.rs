use std::fmt::Display;

use chumsky::{prelude::choice, select, Parser};

use crate::{parser, Prefix, Table, TagType};

#[derive(Debug, Clone)]
pub enum AliasKind {
    Type(String),
    Enum(Vec<(String, Option<String>)>),
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub desc: Vec<String>,
    pub kind: AliasKind,
    pub prefix: Prefix,
}

parser!(Alias, {
    select! {
        TagType::Comment(x) => x,
    }
    .repeated()
    .then(choice((
        select! {
            TagType::Alias(name, Some(ty)) => (name, AliasKind::Type(ty))
        },
        select! { TagType::Alias(name, ..) => name }.then(
            select! {
                TagType::Variant(ty, desc) => (ty, desc)
            }
            .repeated()
            .map(AliasKind::Enum),
        ),
    )))
    .map(|(desc, (name, kind))| Self {
        name,
        desc,
        kind,
        prefix: Prefix::default(),
    })
});

impl Alias {
    pub fn rename_tag(&mut self, tag: String) {
        self.prefix.right = Some(tag);
    }
}

impl Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::{description, header};

        if let Some(prefix) = &self.prefix.right {
            header!(f, self.name, format!("{prefix}.{}", self.name))?;
        } else {
            header!(f, self.name)?;
        }

        if !self.desc.is_empty() {
            description!(f, &self.desc.join("\n"))?;
        }

        writeln!(f)?;

        match &self.kind {
            AliasKind::Type(ty) => {
                description!(f, "Type: ~")?;
                writeln!(f, "{:>w$}", ty, w = 8 + ty.len())?;
            }
            AliasKind::Enum(variants) => {
                description!(f, "Variants: ~")?;

                let mut table = Table::new();
                for (ty, desc) in variants {
                    table.add_row([&format!("({})", ty), desc.as_deref().unwrap_or_default()]);
                }

                write!(f, "{table}")?;
            }
        }

        writeln!(f)
    }
}
