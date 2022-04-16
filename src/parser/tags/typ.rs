use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{child_table, impl_parse, section, TagType};

#[derive(Debug, Clone)]
pub struct Type {
    pub header: Vec<String>,
    pub name: String,
    pub scope: String,
    pub ty: String,
    pub desc: Option<String>,
}

impl_parse!(Type, {
    select! { TagType::Comment(x) => x }
        .repeated()
        .then(select! { TagType::Type(ty, desc) => (ty, desc) })
        .then(select! { TagType::Expr(name, scope) => (name, scope) })
        .map(|((header, (ty, desc)), (name, scope))| Self {
            header,
            name,
            scope,
            ty,
            desc,
        })
});

impl Type {
    pub fn is_public(&self) -> bool {
        &self.scope == "public"
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let detail = child_table!(
            "Type: ~",
            [[
                format!("({})", self.ty).as_str(),
                self.desc.as_deref().unwrap_or_default()
            ]]
        )
        .to_string();

        let section =
            section!(&self.name, &self.name, &self.header.join(" "), [detail]).to_string();

        f.write_str(&section)
    }
}
