use std::fmt::Display;

use crate::parser::Type;

use super::{description, header, see::SeeDoc, usage::UsageDoc, Table};

#[derive(Debug)]
pub struct TypeDoc<'a>(pub &'a Type);

impl Display for TypeDoc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Type {
            desc: (extract, desc),
            prefix,
            name,
            kind,
            ty,
            see,
            usage,
        } = self.0;

        header!(
            f,
            &format!(
                "{}{}{}",
                prefix.left.as_deref().unwrap_or_default(),
                kind.as_char(),
                name
            ),
            &format!(
                "{}{}{}",
                prefix.right.as_deref().unwrap_or_default(),
                kind.as_char(),
                name
            )
        )?;

        if !extract.is_empty() {
            description!(f, &extract.join("\n"))?;
        }

        writeln!(f)?;

        description!(f, "Type: ~")?;

        let mut table = Table::new();
        table.add_row([&format!("({})", ty), desc.as_deref().unwrap_or_default()]);

        writeln!(f, "{table}")?;

        if !see.refs.is_empty() {
            writeln!(f, "{}", SeeDoc(see))?;
        }

        if let Some(usage) = &usage {
            writeln!(f, "{}", UsageDoc(usage))?;
        }

        Ok(())
    }
}
