use std::fmt::Display;

use chumsky::{prelude::choice, select, Parser};

use crate::{parser, Prefix, TagType};

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub ty: String,
    pub desc: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AliasKind {
    Type(TypeDef),
    Enum(Vec<String>, Vec<TypeDef>),
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub kind: AliasKind,
    pub prefix: Prefix,
}

parser!(Alias, {
    choice((
        select! {
            TagType::Alias { name, ty: Some(ty), desc } => {
                Self {
                    name,
                    kind: AliasKind::Type(TypeDef { ty, desc }),
                    prefix: Prefix::default(),
                }
            },
        },
        select! { TagType::Alias { name, .. } => name }
            .then(select! { TagType::Comment(x) => x }.repeated())
            .then(select! { TagType::Variant(ty, desc) => TypeDef { ty, desc } }.repeated())
            .map(|((name, desc), variants)| Self {
                name,
                kind: AliasKind::Enum(desc, variants),
                prefix: Prefix::default(),
            }),
    ))
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

        match &self.kind {
            AliasKind::Type(TypeDef { ty, desc }) => {
                description!(f, desc.as_deref().unwrap_or_default())?;
                writeln!(f)?;
                description!(f, "Type: ~")?;
                writeln!(f, "{:>w$}", ty, w = 8 + ty.len())?;
            }
            AliasKind::Enum(desc, variants) => {
                description!(f, &desc.join("\n"))?;
                writeln!(f)?;
                description!(f, "Variants: ~")?;

                let mut table = tabular::Table::new("        {:<}  {:<}");
                for v in variants {
                    table.add_row(
                        tabular::Row::new()
                            .with_cell(&format!("({})", v.ty))
                            .with_cell(v.desc.as_deref().unwrap_or_default()),
                    );
                }

                write!(f, "{table}")?;
            }
        }

        writeln!(f)
    }
}
