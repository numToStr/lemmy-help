use crate::parser::Usage;

use super::{description, ToDoc};

#[derive(Debug)]
pub struct UsageDoc;

impl ToDoc for UsageDoc {
    type N = Usage;
    fn to_doc(n: &Self::N, _: &super::Settings) -> String {
        let mut doc = String::new();
        doc.push_str(&description("Usage: ~"));
        doc.push('>');
        doc.push_str(n.lang.as_deref().unwrap_or("lua"));
        doc.push('\n');
        doc.push_str(&textwrap::indent(&n.code, "        "));
        doc.push_str("\n<\n\n");
        doc
    }
}
