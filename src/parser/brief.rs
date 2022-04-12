use chumsky::{prelude::just, select, Parser};

use crate::{impl_parse, TagType};

/// ---@brief [[ TEXT @brief ]]
#[derive(Debug)]
pub struct Brief {
    pub desc: Vec<String>,
}

impl_parse!(Brief, {
    select! {
        TagType::Comment(x) => x,
        TagType::Empty => '\n'.into()
    }
    .repeated()
    .delimited_by(just(TagType::BriefStart), just(TagType::BriefEnd))
    .map(|desc| Self { desc })
});
