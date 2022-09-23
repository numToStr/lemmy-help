use chumsky::{
    primitive::{choice, just},
    select, Parser,
};

use crate::{
    lexer::TagType,
    parser::{description, impl_parse},
};

#[derive(Debug, Clone)]
pub struct Usage {
    pub code: String,
}

impl_parse!(Usage, {
    choice((
        select! { TagType::Comment(x) => x }
            .repeated()
            .delimited_by(just(TagType::UsageStart), just(TagType::UsageEnd))
            .map(|x| Self { code: x.join("\n") }),
        select! { TagType::Usage(code) => Self { code } },
    ))
});

impl std::fmt::Display for Usage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        description!(f, "Usage: ~")?;
        writeln!(f, "{:>9}", ">")?;
        writeln!(f, "{}", textwrap::indent(&self.code, "            "))?;
        writeln!(f, "{:>9}", "<")
    }
}
