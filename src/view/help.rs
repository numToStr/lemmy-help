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

#[macro_export]
macro_rules! see {
    ($f:expr, $seee:expr) => {{
        description!($f, "See: ~")?;

        for s in &$seee {
            writeln!($f, "        |{}|", s)?;
        }

        writeln!($f)
    }};
}
