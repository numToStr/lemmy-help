use std::fmt::Display;

use chumsky::select;

use crate::TagType;

// Little helper macro for making parse function
#[macro_export]
macro_rules! impl_parse {
    ($id: ident, $body: expr) => {
        impl $id {
            pub fn parse() -> impl chumsky::Parser<
                crate::TagType,
                Self,
                Error = chumsky::prelude::Simple<crate::TagType>,
            > {
                $body
            }
        }
    };
}

// A TYPE could be
// - primary = string|number|boolean
// - fn = func(...):string
// - enum = "one"|"two"|"three"
// - or: primary (| primary)+
// - optional = primary?
// - table = table<string, string>
// - array = primary[]

/// ---@comment
#[derive(Debug)]
pub struct Comment(pub String);

impl_parse!(Comment, select! { TagType::Comment(x) => Self(x)});

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[macro_export]
macro_rules! section {
    ($name: expr, $tag: expr, $desc: expr, $($data: expr),* $(,)?) => {
        tabled::builder::Builder::from_iter([
            [$name, format!("*{}*", $tag).as_str()],
            [$desc, "".into()],
            ["".into(), "".into()],
            $(
                [$data, "".into()],
            )*
        ]).build()
        .with(tabled::Style::blank())
        .with(tabled::Modify::new(tabled::Full).with(tabled::Padding::new(0, 0, 0, 0)))
        .with(tabled::Modify::new(tabled::Cell(1, 0)).with(tabled::Padding::new(4, 0, 0, 0)))
        .with(tabled::Modify::new(tabled::Columns::new(1..=2)).with(tabled::Alignment::right()))
        .with(tabled::Modify::new(tabled::Columns::new(..1)).with(tabled::Alignment::left()))
        .with(tabled::Modify::new(tabled::Rows::new(1..)).with(tabled::Span::column(2)))
        .with(tabled::MinWidth::new(80))
        .with(tabled::MaxWidth::wrapping(80))
    };
}

#[macro_export]
macro_rules! child_table {
    ($title: expr, $data: expr) => {
        tabled::builder::Builder::from_iter($data)
            .build()
            .with(tabled::Style::blank())
            .with(tabled::Header($title))
            .with(tabled::Footer(""))
            .with(tabled::Margin::new(4, 0, 0, 0))
            .with(
                tabled::Modify::new(tabled::Columns::new(..1))
                    .with(tabled::Padding::new(4, 0, 0, 0)),
            )
            .with(tabled::Modify::new(tabled::Cell(0, 0)).with(tabled::Padding::new(0, 0, 0, 0)))
            .with(tabled::Modify::new(tabled::Full).with(tabled::Alignment::left()))
            .with(
                tabled::Modify::new(tabled::Columns::new(2..)).with(tabled::MaxWidth::wrapping(42)),
            )
    };
}
