use std::fmt::Display;

use chumsky::{select, Parser};
use tabular::{Row, Table};

use crate::{impl_parse, Comment, Object, TagType};

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub desc: Vec<Comment>,
    pub params: Vec<Object>,
    pub returns: Vec<Object>,
}

impl_parse!(Func, {
    Comment::parse()
        .repeated()
        .then(select! { TagType::Param(x) => x }.repeated())
        .then(select! { TagType::Return(x) => x }.repeated())
        .then(select! { TagType::Name(x) => x })
        .map(|(((desc, params), returns), name)| Self {
            name,
            desc,
            params,
            returns,
        })
});

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args: String = self
            .params
            .iter()
            .map(|x| format!("{{{}}}", x.name))
            .collect::<Vec<String>>()
            .join(", ");
        let name = format!("{}({})", self.name, args);

        writeln!(f, "{}\t\t\t\t\t\t\t\t\t*{}*", name, &self.name)?;

        if !self.desc.is_empty() {
            for d in &self.desc {
                writeln!(f, "    {}", d)?;
            }
        }

        writeln!(f)?;

        if !self.params.is_empty() {
            writeln!(f, "    Parameters: ~")?;
            let mut tbl = Table::new("        {:<}  {:<}  {:<}");

            for param in &self.params {
                let row = Row::new()
                    .with_cell(format!("{{{}}}", param.name))
                    .with_cell(format!("({})", param.ty))
                    .with_cell(param.desc.clone().unwrap_or_default());

                tbl.add_row(row);
            }

            writeln!(f, "{}", tbl)?;
        }

        if !self.returns.is_empty() {
            writeln!(f, "    Return: ~")?;
            let mut tbl = Table::new("        {:<}  {:<}  {:<}");

            for param in &self.returns {
                let row = Row::new()
                    .with_cell(format!("{{{}}}", param.name))
                    .with_cell(format!("({})", param.ty))
                    .with_cell(param.desc.clone().unwrap_or_default());

                tbl.add_row(row);
            }

            writeln!(f, "{}", tbl)?;
        }

        write!(f, "")
    }
}
