use crate::parser::See;

use super::{description, ToDoc};

#[derive(Debug)]
pub struct SeeDoc;

impl ToDoc for SeeDoc {
    type N = See;
    fn to_doc(n: &Self::N, _: &super::Settings) -> String {
        let mut doc = String::new();
        doc.push_str(&description("See: ~"));
        for s in &n.refs {
            doc.push_str(&format!("        |{s}|\n"));
        }
        doc.push('\n');
        doc
    }
}
