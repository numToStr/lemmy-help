use crate::parser::Brief;

use super::ToDoc;

#[derive(Debug)]
pub struct BriefDoc;

impl ToDoc for BriefDoc {
    type N = Brief;
    fn to_doc(n: &Self::N, _: &super::Settings) -> String {
        let mut doc = n.desc.join("\n");
        doc.push('\n');
        doc
    }
}
