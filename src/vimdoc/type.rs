use std::fmt::Display;

use crate::parser::Type;

use super::{description, header, see::SeeDoc, usage::UsageDoc};

#[derive(Debug)]
pub struct TypeDoc<'a>(pub &'a Type);

impl Display for TypeDoc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Type {
            desc,
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

        if !desc.is_empty() {
            description!(f, &desc.join("\n"))?;
        }

        writeln!(f)?;

        description!(f, "Type: ~")?;
        f.write_fmt(format_args!(
            "{:>w$}\n\n",
            format!("({})", ty),
            w = 10 + ty.len()
        ))?;

        if !see.refs.is_empty() {
            writeln!(f, "{}", SeeDoc(see))?;
        }

        if let Some(usage) = &usage {
            writeln!(f, "{}", UsageDoc(usage))?;
        }

        Ok(())
    }
}
