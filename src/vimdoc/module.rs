use crate::parser::{Divider, Module};

use super::{divider::DividerDoc, ToDoc};

#[derive(Debug)]
pub struct ModuleDoc;

impl ToDoc for ModuleDoc {
    type N = Module;
    fn to_doc(n: &Self::N, s: &super::Settings) -> String {
        let mut doc = String::new();
        let desc = n.desc.as_deref().unwrap_or_default();
        doc.push_str(&DividerDoc::to_doc(&Divider('='), s));
        doc.push_str(desc);
        doc.push_str(&format!(
            "{:>w$}",
            format!("*{}*", n.name),
            w = 80 - desc.len()
        ));
        doc.push('\n');
        doc
    }
}
