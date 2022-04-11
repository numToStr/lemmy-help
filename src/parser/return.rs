use std::fmt::Display;

use chumsky::{prelude::just, Parser};

use crate::{impl_parse, Desc, Name, Ty};

/// ---@return MY_TYPE[|OTHER_TYPE] [@comment]
#[derive(Debug)]
pub struct Return {
    pub ty: Ty,
    pub name: Name,
    pub desc: Option<Desc>,
}

impl_parse!(Return, {
    just("---@return")
        .ignore_then(Ty::parse())
        .then(Name::parse())
        .then(Desc::parse())
        .map(|((ty, name), desc)| Self { ty, name, desc })
});

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Return: ~")?;

        write!(f, "    {}", &self.ty)?;

        if let Some(desc) = &self.desc {
            writeln!(f, "  {}", desc)
        } else {
            writeln!(f, "  {}", &self.name)
        }
    }
}
