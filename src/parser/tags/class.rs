use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{child_table, impl_parse, section, Object, TagType};

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
    pub fields: Vec<Object>,
    pub see: Vec<String>,
}

impl_parse!(Class, {
    select! { TagType::Class(name, desc) => (name, desc) }
        .then(select! { TagType::Field(x) => x }.repeated())
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
        let mut blocks = Vec::with_capacity(2);

        blocks.push(
            child_table!(
                "Fields: ~",
                self.fields.iter().map(|field| {
                    [
                        format!("{{{}}}", field.ty),
                        format!("({})", field.name),
                        field.desc.clone().unwrap_or_default(),
                    ]
                })
            )
            .to_string(),
        );

        if !self.see.is_empty() {
            blocks.push(
                child_table!("See: ~", self.see.iter().map(|s| [format!("|{}|", s)])).to_string(),
            )
        }

        let desc = self.desc.clone().unwrap_or_default();

        let head = section!(self.name.as_str(), self.name.as_str(), &desc, blocks);

        write!(f, "{}", head)
    }
}
