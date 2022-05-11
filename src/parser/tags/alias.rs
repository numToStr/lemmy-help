use std::fmt::Display;

use chumsky::{prelude::choice, select, Parser};

use crate::{parser, TagType};

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub ty: String,
    pub desc: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AliasKind {
    Type(TypeDef),
    Enum(Vec<TypeDef>),
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub kind: AliasKind,
}

parser!(Alias, {
    choice((
        select! {
            TagType::Alias { name, ty: Some(ty), desc } => {
                Self { name, kind: AliasKind::Type(TypeDef { ty, desc }) }
            },
        },
        select! { TagType::Alias { name, .. } => name }
            .then(
                select! { TagType::Variant(ty, desc) => TypeDef { ty, desc } }
                    .repeated()
                    .map(AliasKind::Enum),
            )
            .map(|(name, kind)| Self { name, kind }),
    ))
});

impl Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::{description, header};

        header!(f, self.name)?;

        match &self.kind {
            AliasKind::Type(TypeDef { ty, desc }) => {
                description!(f, desc.as_deref().unwrap_or_default())?;
                writeln!(f)?;
                description!(f, "Type: ~")?;
                writeln!(f, "{:>w$}", ty, w = 8 + ty.len())?;
            }
            AliasKind::Enum(variants) => {
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
