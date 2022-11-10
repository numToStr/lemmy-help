use crate::{lexer::Name, parser::Func, Layout};

use super::{description, header, see::SeeDoc, usage::UsageDoc, Table, ToDoc};

#[derive(Debug)]
pub struct FuncDoc;

impl ToDoc for FuncDoc {
    type N = Func;
    fn to_doc(n: &Self::N, s: &super::Settings) -> String {
        let mut doc = String::new();

        let name_with_param = if !n.params.is_empty() {
            let args = n
                .params
                .iter()
                .map(|p| format!("{{{}}}", p.name))
                .collect::<Vec<String>>()
                .join(", ");

            format!(
                "{}{}{}({args})",
                n.prefix.left.as_deref().unwrap_or_default(),
                n.kind.as_char(),
                n.name
            )
        } else {
            format!(
                "{}{}{}()",
                n.prefix.left.as_deref().unwrap_or_default(),
                n.kind.as_char(),
                n.name
            )
        };

        doc.push_str(&header!(
            name_with_param,
            &format!(
                "{}{}{}",
                n.prefix.right.as_deref().unwrap_or_default(),
                n.kind.as_char(),
                n.name,
            )
        ));

        if !n.desc.is_empty() {
            doc.push_str(&description(&n.desc.join("\n")))
        }

        doc.push('\n');

        if !n.params.is_empty() {
            doc.push_str(&description("Parameters: ~"));

            let mut table = Table::new();

            for param in &n.params {
                let (name, ty) = match (s.expand_opt, &param.name) {
                    (true, Name::Opt(n)) => (format!("{{{n}}}"), format!("(nil|{})", param.ty)),
                    (_, n) => (format!("{{{n}}}"), format!("({})", param.ty)),
                };

                if let Layout::Compact(x) = s.layout {
                    table.add_row([
                        name,
                        format!(
                            "{ty} {}",
                            param.desc.join(&format!("\n{}", " ".repeat(x as usize)))
                        ),
                    ]);
                } else {
                    table.add_row([name, ty, param.desc.join("\n")]);
                }
            }

            doc.push_str(&table.to_string());
            doc.push('\n');
        }

        if !n.returns.is_empty() {
            doc.push_str(&description("Returns: ~"));

            let mut table = Table::new();

            for entry in &n.returns {
                table.add_row([
                    format!("{{{}}}", entry.ty),
                    if entry.desc.is_empty() {
                        entry.name.clone().unwrap_or_default()
                    } else {
                        entry.desc.join("\n")
                    },
                ]);
            }

            doc.push_str(&table.to_string());
            doc.push('\n');
        }

        if !n.see.refs.is_empty() {
            doc.push_str(&SeeDoc::to_doc(&n.see, s));
        }

        if let Some(usage) = &n.usage {
            doc.push_str(&UsageDoc::to_doc(usage, s));
        }

        doc
    }
}
