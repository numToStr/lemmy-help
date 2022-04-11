use chumsky::select;

use crate::{impl_parse, CommentType};

#[derive(Debug)]
pub struct Tag(String);

impl_parse!(Tag, {
    select! { CommentType::Tag(x) => Self(x) }
});
