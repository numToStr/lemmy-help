use std::fmt::Display;

use chumsky::{prelude::just, Parser};
use tabular::{Row, Table};

use crate::{impl_parse, Comment, Field, Name};

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
    pub name: Name,
    pub desc: Option<Comment>,
    pub fields: Vec<Field>,
}

impl_parse!(Class, {
    just("@class")
        .ignore_then(Name::parse())
        .then(Comment::parse())
        .then(Field::parse().repeated())
        .map(|((name, desc), fields)| Self { name, desc, fields })
});

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{n}\t\t\t\t\t\t\t\t\t*{n}*", n = &self.name)?;

        if let Some(x) = &self.desc {
            writeln!(f, "    {}", x)?;
        }

        writeln!(f)?;
        writeln!(f, "    Fields: ~")?;

        let mut tbl = Table::new("        {:<}  {:<}  {:<}");

        for f in &self.fields {
            let row = Row::new()
                .with_cell(format!("{{{}}}", f.name))
                .with_cell(format!("({})", f.ty))
                .with_cell(f.desc.clone().unwrap_or_default());

            tbl.add_row(row);
        }

        f.write_str(&tbl.to_string())
    }
}
