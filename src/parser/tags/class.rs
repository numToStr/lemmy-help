use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{parser, Prefix, Scope, See, Table, TagType};

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub desc: Option<String>,
    pub fields: Vec<Field>,
    pub see: See,
    pub prefix: Prefix,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub scope: Scope,
    pub name: String,
    pub ty: String,
    pub desc: Option<String>,
}

parser!(Class, {
    select! { TagType::Class(name, desc) => (name, desc) }
        .then(
            select! {
                TagType::Field { scope, name, ty, desc } => Field { scope, name, ty, desc }
            }
            .repeated(),
        )
        .then(See::parse())
        .map(|(((name, desc), fields), see)| Self {
            name,
            desc,
            fields,
            see,
            prefix: Prefix::default(),
        })
});

impl Class {
    pub fn rename_tag(&mut self, tag: String) {
        self.prefix.right = Some(tag);
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::{description, header};

        if let Some(prefix) = &self.prefix.right {
            header!(f, self.name, format!("{prefix}.{}", self.name))?;
        } else {
            header!(f, self.name)?;
        }

        description!(f, self.desc.as_deref().unwrap_or_default())?;
        writeln!(f)?;

        if !self.fields.is_empty() {
            description!(f, "Fields: ~")?;

            let mut table = Table::new();

            for field in &self.fields {
                if field.scope == Scope::Public {
                    table.add_row([
                        &format!("{{{}}}", field.name),
                        &format!("({})", field.ty),
                        field.desc.as_deref().unwrap_or_default(),
                    ]);
                }
            }

            writeln!(f, "{table}")?;
        }

        if !self.see.refs.is_empty() {
            writeln!(f, "{}", self.see)?;
        }

        write!(f, "")
    }
}
