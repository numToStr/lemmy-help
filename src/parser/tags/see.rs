use chumsky::{select, Parser};

use crate::{
    lexer::TagType,
    parser::{description, impl_parse},
};

#[derive(Debug, Clone)]
pub struct See {
    pub refs: Vec<String>,
}

impl_parse!(See, {
    select! { TagType::See(x) => x }
        .repeated()
        .map(|refs| Self { refs })
});

impl std::fmt::Display for See {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        description!(f, "See: ~")?;
        for s in &self.refs {
            writeln!(f, "        |{s}|")?;
        }
        Ok(())
    }
}
