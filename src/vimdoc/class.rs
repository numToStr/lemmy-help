use std::fmt::Display;

use crate::{lexer::Scope, parser::Class};

use super::{description, header, see::SeeDoc, Table};

#[derive(Debug)]
pub struct ClassDoc<'a>(pub &'a Class);

impl Display for ClassDoc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Class {
            name,
            desc,
            fields,
            see,
            prefix,
        } = self.0;

        if let Some(prefix) = &prefix.right {
            header!(f, name, format!("{prefix}.{}", name))?;
        } else {
            header!(f, name)?;
        }

        if !desc.is_empty() {
            description!(f, &desc.join("\n"))?;
        }
        writeln!(f)?;

        if !fields.is_empty() {
            description!(f, "Fields: ~")?;

            let mut table = Table::new();

            for field in fields {
                if field.scope == Scope::Public {
                    table.add_row([
                        &format!("{{{}}}", field.name),
                        &format!("({})", field.ty),
                        &field.desc.join("\n"),
                    ]);
                }
            }

            writeln!(f, "{table}")?;
        }

        if !see.refs.is_empty() {
            writeln!(f, "{}", SeeDoc(see))?;
        }

        Ok(())
    }
}
