use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{child_table, impl_parse, section, Name, TagType};

#[derive(Debug, Clone)]
pub struct Type {
    pub header: Vec<String>,
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
            name,
            scope,
            ty,
            desc,
            usage,
        })
});

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut blocks = Vec::with_capacity(2);

        blocks.push(
            child_table!(
                "Type: ~",
                [[
                    format!("({})", self.ty).as_str(),
                    self.desc.as_deref().unwrap_or_default()
                ]]
            )
            .to_string(),
        );

        if let Some(usage) = &self.usage {
            blocks.push(
                child_table!("Usage: ~", [[">"], [&format!("  {}", usage)], ["<"]]).to_string(),
            )
        }

        let name = self.name.to_string();
        let desc = self.header.join(" ");

        let section = section!(&name, &name, &desc, blocks).to_string();

        f.write_str(&section)
    }
}
