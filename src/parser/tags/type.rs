use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{parser, Kind, Prefix, Table, TagType, Usage};

#[derive(Debug, Clone)]
pub struct Type {
    pub header: Vec<String>,
    pub prefix: Prefix,
    pub name: String,
    pub kind: Kind,
    pub ty: String,
    pub desc: Option<String>,
    pub usage: Option<Usage>,
}

parser!(Type, {
    select! { TagType::Comment(x) => x }
        .repeated()
        .then(select! { TagType::Type(ty, desc) => (ty, desc) })
        .then(Usage::parse().or_not())
        .then(select! { TagType::Expr { prefix, name, kind } => (prefix, name, kind) })
        .map(
            |(((header, (ty, desc)), usage), (prefix, name, kind))| Self {
                header,
                prefix: Prefix {
                    left: prefix.clone(),
                    right: prefix,
                },
                name,
                kind,
                ty,
                desc,
                usage,
            },
        )
});

impl Type {
    pub fn rename_tag(&mut self, tag: String) {
        self.prefix.right = Some(tag);
    }

    pub fn is_public(&self, export: &str) -> bool {
        self.kind != Kind::Local && self.prefix.left.as_deref() == Some(export)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::{description, header};

        header!(
            f,
            format!(
                "{}{}{}",
                self.prefix.left.as_deref().unwrap_or_default(),
                self.kind.as_char(),
                self.name
            ),
            format!(
                "{}{}{}",
                self.prefix.right.as_deref().unwrap_or_default(),
                self.kind.as_char(),
                self.name
            )
        )?;

        description!(f, &self.header.join("\n"))?;
        writeln!(f)?;

        description!(f, "Type: ~")?;

        let mut table = Table::new();

        table.add_row([
            &format!("({})", self.ty),
            self.desc.as_deref().unwrap_or_default(),
        ]);

        writeln!(f, "{table}")?;

        if let Some(usage) = &self.usage {
            writeln!(f, "{usage}")?;
        }

        Ok(())
    }
}
