use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{impl_parse, usage, Name, TagType};

#[derive(Debug, Clone)]
pub struct Type {
    pub header: Vec<String>,
    pub tag: Name,
    pub name: Name,
    pub scope: String,
    pub ty: String,
    pub desc: Option<String>,
    pub usage: Option<String>,
}

impl_parse!(Type, {
    select! { TagType::Comment(x) => x }
        .repeated()
        .then(select! { TagType::Type(ty, desc) => (ty, desc) })
        .then(select! { TagType::Usage(x) => x }.or_not())
        .then(select! { TagType::Expr(name, scope) => (name, scope) })
        .map(|(((header, (ty, desc)), usage), (name, scope))| Self {
            header,
            tag: name.clone(),
            name,
            scope,
            ty,
            desc,
            usage,
        })
});

impl Type {
    pub fn rename_tag(mut self, tag: String) -> Self {
        if let Name::Member(_, field, kind) = self.tag {
            self.tag = Name::Member(tag, field, kind)
        };

        self
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::{description, header};

        header!(f, self.name.to_string(), self.tag.to_string())?;
        description!(f, &self.header.join("\n"))?;
        writeln!(f)?;

        description!(f, "Type:~")?;

        let mut table = tabular::Table::new("        {:<}  {:<}");

        table.add_row(
            tabular::Row::new()
                .with_cell(&format!("({})", self.ty))
                .with_cell(self.desc.as_deref().unwrap_or_default()),
        );

        writeln!(f, "{}", table)?;

        if let Some(usage) = &self.usage {
            usage!(f, usage)?;
        }

        write!(f, "")
    }
}
