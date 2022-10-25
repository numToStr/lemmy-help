use crate::parser::Tag;

use super::ToDoc;

#[derive(Debug)]
pub struct TagDoc;

impl ToDoc for TagDoc {
    type N = Tag;
    fn to_doc(n: &Self::N, _: &super::Settings) -> String {
        format!("{:>80}", format!("*{}*", n.0))
    }
}
