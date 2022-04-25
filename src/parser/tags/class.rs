use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{impl_parse, see, TagType};

/// **Grammar**
///
/// ---@class MY_TYPE[:PARENT_TYPE] [@comment]
///
/// **Emmy**
///
/// ---@class CMode Comment modes - Can be manual or computed in operator-pending phase
/// ---@field toggle number Toggle action
/// ---@field comment number Comment action
/// ---@field uncomment number Uncomment action
///
/// **Help**
///
/// CMode                                                                   \*CMode\*
///     Comment modes - Can be manual or computed in operator-pending phase
///
///     Fields: ~
///         {toggle}     (number)    Toggle action
///         {comment}    (number)    Comment action
///         {uncomment}  (number)    Uncomment action
///
#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub desc: Option<String>,
    pub fields: Vec<Field>,
    pub see: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: String,
    pub desc: Option<String>,
}

impl_parse!(Class, {
    select! { TagType::Class(name, desc) => (name, desc) }
        .then(select! { TagType::Field { name, ty, desc } => Field { name, ty, desc } }.repeated())
        .then(select! { TagType::See(x) => x }.repeated())
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

        if !self.see.is_empty() {
            see!(f, self.see)?;
        }

        write!(f, "")
    }
}
