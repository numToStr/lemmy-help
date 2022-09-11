pub struct Table(comfy_table::Table);

impl Table {
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

macro_rules! impl_parse {
    ($id: ident, $ret: ty, $body: expr) => {
        impl $id {
            pub fn parse() -> impl chumsky::Parser<
                $crate::lexer::TagType,
                $ret,
                Error = chumsky::prelude::Simple<$crate::lexer::TagType>,
            > {
                $body
            }
        }
    };
    ($id: ident, $body: expr) => {
        crate::parser::impl_parse!($id, Self, $body);
    };
}

pub(super) use impl_parse;

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
        crate::parser::header!($f, $name, $name)
    };
}

pub(super) use header;

macro_rules! description {
    ($f:expr, $desc:expr) => {
        writeln!($f, "{}", textwrap::indent($desc, "    "))
    };
}

pub(super) use description;
