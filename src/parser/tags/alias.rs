use std::fmt::Display;

use chumsky::select;

use crate::{impl_parse, TagType};

#[derive(Debug, Clone)]
pub struct Alias {
    name: String,
    ty: String,
    desc: Option<String>,
}

impl_parse!(Alias, {
    select! { TagType::Alias { name, ty, desc } => Self { name, ty, desc } }
});

impl Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::{description, header};

        header!(f, self.name)?;
        description!(f, self.desc.as_deref().unwrap_or_default())?;
        writeln!(f)?;
        description!(f, "Type:~")?;
        writeln!(f, "{:>w$}", self.ty, w = 8 + self.ty.len())?;
        writeln!(f)
    }
}
