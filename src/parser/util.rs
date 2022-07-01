pub struct Table(comfy_table::Table);

impl Table {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut tbl = comfy_table::Table::new();
        tbl.load_preset(comfy_table::presets::NOTHING);
        // tbl.column_iter_mut().map(|c| c.set_padding((0, 0)));

        Self(tbl)
    }

    pub fn add_row<T: Into<comfy_table::Row>>(&mut self, row: T) -> &Self {
        self.0.add_row(row);
        self
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", textwrap::indent(&self.0.trim_fmt(), "       "))
    }
}

#[macro_export]
macro_rules! parser {
    ($id: ident, $ret: ty, $body: expr) => {
        impl $id {
            pub fn parse() -> impl chumsky::Parser<
                crate::TagType,
                $ret,
                Error = chumsky::prelude::Simple<crate::TagType>,
            > {
                $body
            }
        }
    };
    ($id: ident, $body: expr) => {
        crate::parser!($id, Self, $body);
    };
}

#[macro_export]
macro_rules! header {
    ($f:expr, $name:expr, $tag:expr) => {{
        let len = $name.len();
        if len > 40 || $tag.len() > 40 {
            writeln!($f, "{:>80}", format!("*{}*", $tag))?;
            writeln!($f, "{}", $name)
        } else {
            writeln!(
                $f,
                "{}{}",
                $name,
                format_args!("{:>w$}", format!("*{}*", $tag), w = 80 - len)
            )
        }
    }};
    ($f:expr, $name:expr) => {
        crate::header!($f, $name, $name)
    };
}

#[macro_export]
macro_rules! description {
    ($f:expr, $desc:expr) => {
        writeln!($f, "{}", textwrap::indent($desc, "    "))
    };
}
