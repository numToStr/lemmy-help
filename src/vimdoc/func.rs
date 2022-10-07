use std::fmt::Display;

use crate::parser::Func;

use super::{description, header, see::SeeDoc, usage::UsageDoc, Table};

#[derive(Debug)]
pub struct FuncDoc<'a>(pub &'a Func);

impl Display for FuncDoc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Func {
            name,
            kind,
            prefix,
            desc,
            params,
            returns,
            see,
            usage,
        } = self.0;

        let is_opt = |opt| if opt { "?" } else { "" };

        let name_with_param = if !params.is_empty() {
            let args = params
                .iter()
                .map(|param| format!("{{{}{}}}", param.name, is_opt(param.optional)))
                .collect::<Vec<String>>()
                .join(", ");

            format!("{}({})", name, args)
        } else {
            format!("{}()", name)
        };

        header!(
            f,
            &format!(
                "{}{}{name_with_param}",
                prefix.left.as_deref().unwrap_or_default(),
                kind.as_char()
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

        if !params.is_empty() {
            description!(f, "Parameters: ~")?;

            let mut table = Table::new();

            for param in params {
                table.add_row([
                    format!("{{{}{}}}", param.name, is_opt(param.optional)),
                    format!("({})", param.ty),
                    param.desc.join("\n"),
                ]);
            }

            writeln!(f, "{table}")?;
        }

        if !returns.is_empty() {
            description!(f, "Returns: ~")?;

            let mut table = Table::new();

            for entry in returns {
                table.add_row([
                    format!("{{{}}}", entry.ty),
                    if entry.desc.is_empty() {
                        entry.name.clone().unwrap_or_default()
                    } else {
                        entry.desc.join("\n")
                    },
                ]);
            }

            writeln!(f, "{table}")?;
        }

        if !see.refs.is_empty() {
            writeln!(f, "{}", SeeDoc(see))?;
        }

        if let Some(usage) = &usage {
            writeln!(f, "{}", UsageDoc(usage))?;
        }

        Ok(())
    }
}
