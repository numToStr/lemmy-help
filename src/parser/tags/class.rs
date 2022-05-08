use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{parser, See, TagType};

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub desc: Option<String>,
    pub fields: Vec<Field>,
    pub see: See,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: String,
    pub desc: Option<String>,
}

parser!(Class, {
    select! { TagType::Class(name, desc) => (name, desc) }
        .then(select! { TagType::Field { name, ty, desc } => Field { name, ty, desc } }.repeated())
        .then(See::parse())
        .map(|(((name, desc), fields), see)| Self {
            name,
            desc,
            fields,
            see,
        })
});

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::{description, header};

        header!(f, self.name)?;
        description!(f, self.desc.as_deref().unwrap_or_default())?;
        writeln!(f)?;

        if !self.fields.is_empty() {
            description!(f, "Fields: ~")?;

            let mut table = tabular::Table::new("        {:<}  {:<}  {:<}");

            for field in &self.fields {
                table.add_row(
                    tabular::Row::new()
                        .with_cell(&format!("{{{}}}", field.name))
                        .with_cell(&format!("({})", field.ty))
                        .with_cell(field.desc.as_deref().unwrap_or_default()),
                );
            }

            writeln!(f, "{}", table)?;
        }

        if !self.see.refs.is_empty() {
            writeln!(f, "{}", self.see)?;
        }

        write!(f, "")
    }
}
