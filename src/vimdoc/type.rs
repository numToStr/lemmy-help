use crate::parser::Type;

use super::{description, header, see::SeeDoc, usage::UsageDoc, Table, ToDoc};

#[derive(Debug)]
pub struct TypeDoc;

impl ToDoc for TypeDoc {
    type N = Type;
    fn to_doc(n: &Self::N, s: &super::Settings) -> String {
        let mut doc = String::new();

        doc.push_str(&header!(
            &format!("{}{}", n.prefix.left.as_deref().unwrap_or_default(), n.op),
            &format!("{}{}", n.prefix.right.as_deref().unwrap_or_default(), n.op)
        ));

        let (extract, desc) = &n.desc;

        if !extract.is_empty() {
            doc.push_str(&description(&extract.join("\n")));
        }

        doc.push('\n');

        doc.push_str(&description("Type: ~"));

        let mut table = Table::new();
        table.add_row([&format!("({})", n.ty), desc.as_deref().unwrap_or_default()]);
        doc.push_str(&table.to_string());
        doc.push('\n');

        if !n.see.refs.is_empty() {
            doc.push_str(&SeeDoc::to_doc(&n.see, s));
        }

        if let Some(usage) = &n.usage {
            doc.push_str(&UsageDoc::to_doc(usage, s));
        }

        doc
    }
}
