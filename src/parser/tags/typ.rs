use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{child_table, impl_parse, section, TagType};

#[derive(Debug)]
pub struct Type {
    pub header: Vec<String>,
    pub ty: String,
    pub desc: Option<String>,
}

impl_parse!(Type, {
    select! { TagType::Comment(x) => x }
        .repeated()
        .then(select! { TagType::Type(name, desc) => (name, desc) })
        .map(|(header, (ty, desc))| Self { header, ty, desc })
});

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

        let section = section!("type", "type", &self.header.join(" "), [detail]).to_string();

        f.write_str(&section)
    }
}
