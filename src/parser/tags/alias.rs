use std::fmt::Display;

use chumsky::select;

use crate::{impl_parse, Object, TagType};

#[derive(Debug, Clone)]
pub struct Alias(Object);

impl_parse!(Alias, {
    select! {TagType::Alias(x) => Self(x)}
});

impl Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::{description, header};

        let a = &self.0;

        header!(f, a.name)?;
        description!(f, a.desc.as_deref().unwrap_or_default())?;
        writeln!(f)?;
        description!(f, "Type:~")?;
        writeln!(f, "{:>w$}", a.ty, w = 8 + a.ty.len())?;
        writeln!(f)
    }
}
