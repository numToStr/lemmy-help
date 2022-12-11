use crate::parser::{Alias, AliasKind};

use super::{description, header, Table, ToDoc};

#[derive(Debug)]
pub struct AliasDoc;

impl ToDoc for AliasDoc {
    type N = Alias;
    fn to_doc(n: &Self::N, _: &super::Settings) -> String {
        let mut doc = String::new();

        if let Some(prefix) = &n.prefix.right {
            doc.push_str(&header(&n.name, &format!("{prefix}.{}", n.name)));
        } else {
            doc.push_str(&header(&n.name, &n.name));
        }

        if !n.desc.is_empty() {
            doc.push_str(&description(&n.desc.join("\n")));
        }

        doc.push('\n');

        match &n.kind {
            AliasKind::Type(ty) => {
                doc.push_str(&description("Type: ~"));
                let ty = ty.to_string();
                doc.push_str(&format!("{:>w$}", ty, w = 8 + ty.len()));
                doc.push('\n');
            }
            AliasKind::Enum(variants) => {
                doc.push_str(&description("Variants: ~"));
                let mut table = Table::new();
                for (ty, desc) in variants {
                    table.add_row([&format!("({})", ty), desc.as_deref().unwrap_or_default()]);
                }
                doc.push_str(&table.to_string());
            }
        }

        doc.push('\n');
        doc
    }
}
