use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{
    lexer::{Scope, TagType},
    parser::{description, header, impl_parse, Prefix, See, Table},
};

#[derive(Debug, Clone)]
pub struct Field {
    pub scope: Scope,
    pub name: String,
    pub ty: String,
    pub desc: Vec<String>,
}

impl_parse!(Field, {
    select! {
        TagType::Comment(x) => x,
    }
    .repeated()
    .then(select! {
        TagType::Field { scope, name, ty, desc } => (scope, name, ty, desc)
    })
    .map(|(header, (scope, name, ty, desc))| {
        let desc = match desc {
            Some(d) => vec![d],
            None => header,
        };

        Self {
            scope,
            name,
            ty,
            desc,
        }
    })
});

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub desc: Vec<String>,
    pub fields: Vec<Field>,
    pub see: See,
    pub prefix: Prefix,
}

impl_parse!(Class, {
    select! { TagType::Comment(c) => c }
        .repeated()
        .then(select! { TagType::Class(name) => name })
        .then(Field::parse().repeated())
        .then(See::parse())
        .map(|(((desc, name), fields), see)| Self {
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
        if let Some(prefix) = &self.prefix.right {
            header!(f, &self.name, format!("{prefix}.{}", self.name))?;
        } else {
            header!(f, &self.name)?;
        }

        if !self.desc.is_empty() {
            description!(f, &self.desc.join("\n"))?;
        }
        writeln!(f)?;

        if !self.fields.is_empty() {
            description!(f, "Fields: ~")?;

            let mut table = Table::new();

            for field in &self.fields {
                if field.scope == Scope::Public {
                    table.add_row([
                        &format!("{{{}}}", field.name),
                        &format!("({})", field.ty),
                        &field.desc.join("\n"),
                    ]);
                }
            }

            writeln!(f, "{table}")?;
        }

        if !self.see.refs.is_empty() {
            writeln!(f, "{}", self.see)?;
        }

        Ok(())
    }
}
