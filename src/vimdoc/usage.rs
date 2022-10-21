use crate::parser::Usage;

use super::{description, ToDoc};

#[derive(Debug)]
pub struct UsageDoc;

impl ToDoc for UsageDoc {
    type N = Usage;
    fn to_doc(n: &Self::N, _: &super::Settings) -> String {
        let mut doc = String::new();
        doc.push_str(&description("Usage: ~"));
        doc.push_str("        >\n");
        doc.push_str(&textwrap::indent(&n.code, "            "));
        doc.push('\n');
        doc.push_str("        <\n\n");
        doc
    }
}
