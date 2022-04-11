use chumsky::{prelude::just, select, Parser};

use crate::{impl_parse2, CommentType};

/// ---@brief [[ TEXT @brief ]]
#[derive(Debug)]
pub struct Brief {
    pub desc: Vec<String>,
}

impl_parse2!(Brief, {
    select! {
        CommentType::Str(x) => x,
        CommentType::Empty => '\n'.into()
    }
    .repeated()
    .delimited_by(just(CommentType::BriefStart), just(CommentType::BriefEnd))
    .map(|desc| Self { desc })
});
