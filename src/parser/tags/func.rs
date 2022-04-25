use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{impl_parse, see, usage, Name, Object, TagType};

#[derive(Debug, Clone)]
pub struct Param {
    name: String,
    ty: String,
    desc: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub tag: Name,
    pub name: Name,
    pub scope: String,
    pub desc: Vec<String>,
    pub params: Vec<Param>,
    pub returns: Vec<Object>,
    pub see: Vec<String>,
    pub usage: Option<String>,
}

impl_parse!(Func, {
    select! {
        TagType::Comment(x) => x,
        TagType::Empty => "\n".to_string()
    }
    .repeated()
    .then(select! { TagType::Param { name, ty, desc } => Param { name, ty, desc } }.repeated())
    .then(select! { TagType::Return(x) => x }.repeated())
    .then(select! { TagType::See(x) => x }.repeated())
    .then(select! { TagType::Usage(x) => x }.or_not())
    .then(select! { TagType::Func(n, s) => (n, s) })
    .map(
        |(((((desc, params), returns), see), usage), (name, scope))| Self {
            tag: name.clone(),
            name,
            scope,
            desc,
            params,
            returns,
            see,
            usage,
        },
    )
});

impl Func {
    pub fn rename_tag(mut self, tag: String) -> Self {
        if let Name::Member(_, field, kind) = self.tag {
            self.tag = Name::Member(tag, field, kind)
        };

        self
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::{description, header};

        let name = if !self.params.is_empty() {
            let args = self
                .params
                .iter()
                .map(|x| format!("{{{}}}", x.name))
                .collect::<Vec<String>>()
                .join(", ");

            format!("{}({})", self.name, args)
        } else {
            format!("{}()", self.name)
        };

        let desc = self.desc.join("\n");

        header!(f, name, self.tag.to_string())?;
        description!(f, &desc)?;
        writeln!(f)?;

        if !self.params.is_empty() {
            description!(f, "Parameters: ~")?;

            let mut table = tabular::Table::new("        {:<}  {:<}  {:<}");

            for param in &self.params {
                table.add_row(
                    tabular::Row::new()
                        .with_cell(&format!("{{{}}}", param.name))
                        .with_cell(&format!("({})", param.ty))
                        .with_cell(param.desc.as_deref().unwrap_or_default()),
                );
            }

            writeln!(f, "{}", table)?;
        }

        if !self.returns.is_empty() {
            description!(f, "Returns: ~")?;

            let mut table = tabular::Table::new("        {:<}  {:<}");

            for entry in &self.returns {
                table.add_row(
                    tabular::Row::new()
                        .with_cell(&format!("{{{}}}", entry.ty))
                        .with_cell(entry.desc.as_deref().unwrap_or(&entry.name)),
                );
            }

            writeln!(f, "{}", table)?;
        }

        if !self.see.is_empty() {
            see!(f, self.see)?;
        }

        if let Some(usage) = &self.usage {
            usage!(f, usage)?;
        }

        write!(f, "")
    }
}
