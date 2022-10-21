use crate::{
    lexer::{Name, Scope},
    parser::Class,
};

use super::{description, header, see::SeeDoc, Table, ToDoc};

#[derive(Debug)]
pub struct ClassDoc;

impl ToDoc for ClassDoc {
    type N = Class;
    fn to_doc(n: &Self::N, s: &super::Settings) -> String {
        let mut doc = String::new();

        if let Some(prefix) = &n.prefix.right {
            doc.push_str(&header!(n.name, format!("{prefix}.{}", n.name)));
        } else {
            doc.push_str(&header!(n.name));
        }

        if !n.desc.is_empty() {
            doc.push_str(&description(&n.desc.join("\n")));
        }
        doc.push('\n');

        if !n.fields.is_empty() {
            doc.push_str(&description("Fields: ~"));

            let mut table = Table::new();

            for field in &n.fields {
                let n = match (s.expand_opt, &field.name) {
                    (true, Name::Req(n) | Name::Opt(n)) => format!("{{{n}}}"),
                    (false, n) => format!("{{{n}}}"),
                };

                let t = if s.expand_opt {
                    format!("(nil|{})", field.ty)
                } else {
                    format!("({})", field.ty)
                };

                if field.scope == Scope::Public {
                    table.add_row([n, t, field.desc.join("\n")]);
                }
            }

            doc.push_str(&table.to_string());
            doc.push('\n');
        }

        if !n.see.refs.is_empty() {
            doc.push_str(&SeeDoc::to_doc(&n.see, s));
        }

        doc
    }
}
