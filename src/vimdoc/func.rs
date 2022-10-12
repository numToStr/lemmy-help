use std::fmt::Display;

use crate::{
    lexer::{Ty, TypeVal},
    parser::{Func, Param},
};

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

        fn is_opt(typeval: &'_ TypeVal) -> (String, &Ty) {
            match typeval {
                TypeVal::Opt(k, v) => (format!("{{{k}?}}"), v),
                TypeVal::Req(k, v) => (format!("{{{k}}}"), v),
            }
        }

        let name_with_param = if !params.is_empty() {
            let args = params
                .iter()
                .map(|Param(typeval, _)| is_opt(typeval).0)
                .collect::<Vec<String>>()
                .join(", ");

            format!(
                "{}{}{name}({args})",
                prefix.left.as_deref().unwrap_or_default(),
                kind.as_char()
            )
        } else {
            format!(
                "{}{}{name}()",
                prefix.left.as_deref().unwrap_or_default(),
                kind.as_char()
            )
        };

        header!(
            f,
            name_with_param,
            &format!(
                "{}{}{name}",
                prefix.right.as_deref().unwrap_or_default(),
                kind.as_char(),
            )
        )?;

        if !desc.is_empty() {
            description!(f, &desc.join("\n"))?;
        }

        writeln!(f)?;

        if !params.is_empty() {
            description!(f, "Parameters: ~")?;

            let mut table = Table::new();

            for Param(typeval, desc) in params {
                let (name, ty) = is_opt(typeval);
                table.add_row([name, format!("({ty})"), desc.join("\n")]);
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
