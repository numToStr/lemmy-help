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
#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub desc: Option<String>,
    pub fields: Vec<Object>,
}

impl_parse!(Class, {
    select! { TagType::Class(name, desc) => (name, desc) }
        .then(select! { TagType::Field(x) => x }.repeated())
        .map(|((name, desc), fields)| Self { name, desc, fields })
});

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fields = child_table!(
            "Fields: ~",
            self.fields.iter().map(|field| {
                [
                    format!("{{{}}}", field.ty),
                    format!("({})", field.name),
                    field.desc.clone().unwrap_or_default(),
                ]
            })
        );

        let head = section!(
            self.name.as_str(),
            self.name.as_str(),
            self.desc.clone().unwrap_or_default().as_str(),
            fields.to_string().as_str()
        );

        write!(f, "{}", head)
    }
}
