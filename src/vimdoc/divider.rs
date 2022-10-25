use crate::parser::Divider;

use super::ToDoc;

#[derive(Debug)]
pub struct DividerDoc;

impl ToDoc for DividerDoc {
    type N = Divider;
    fn to_doc(n: &Self::N, _: &super::Settings) -> String {
        let mut s = String::with_capacity(81);
        for _ in 0..80 {
            s.push(n.0);
        }
        s.push('\n');
        s
    }
}
