use std::fmt::Display;

use crate::parser::{Alias, AliasKind};

use super::{description, header, Table};

#[derive(Debug)]
pub struct AliasDoc<'a>(pub &'a Alias);

impl Display for AliasDoc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Alias {
            prefix,
            name,
            desc,
            kind,
        } = &self.0;

        if let Some(prefix) = &prefix.right {
            header!(f, name, format!("{prefix}.{}", name))?;
        } else {
            header!(f, name)?;
        }

        if !desc.is_empty() {
            description!(f, &desc.join("\n"))?;
        }

        writeln!(f)?;

        match &kind {
            AliasKind::Type(ty) => {
                description!(f, "Type: ~")?;
                let ty = ty.to_string();
                writeln!(f, "{:>w$}", ty, w = 8 + ty.len())?;
            }
            AliasKind::Enum(variants) => {
                description!(f, "Variants: ~")?;

                let mut table = Table::new();
                for (ty, desc) in variants {
                    table.add_row([&format!("({})", ty), desc.as_deref().unwrap_or_default()]);
                }

                f.write_str(&table.to_string())?;
            }
        }

        writeln!(f)
    }
}
