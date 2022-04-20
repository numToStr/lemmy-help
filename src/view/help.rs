#[macro_export]
macro_rules! section {
    ($name: expr, $tag: expr, $desc: expr, $data: expr) => {{
        let mut rows = vec![];

        let tag = format!("*{}*", $tag);

        let to_indent = if $name.len() > 45 {
            rows.push(["", &tag]);
            rows.push([$name, ""]);
            2
        } else {
            rows.push([$name, &tag]);
            1
        };

        rows.push([$desc, ""]);
        rows.push(["", ""]);

        tabled::builder::Builder::from_iter(
            rows.into_iter()
                .chain($data.iter().map(|x| [x.as_str(), ""])),
        )
        .build()
        .with(tabled::Style::blank())
        .with(tabled::Modify::new(tabled::Full).with(tabled::Padding::new(0, 0, 0, 0)))
        .with(
            tabled::Modify::new(tabled::Cell(to_indent, 0)).with(tabled::Padding::new(4, 0, 0, 0)),
        )
        .with(tabled::Modify::new(tabled::Columns::new(1..=2)).with(tabled::Alignment::right()))
        .with(tabled::Modify::new(tabled::Columns::new(..1)).with(tabled::Alignment::left()))
        .with(tabled::Modify::new(tabled::Rows::new(1..)).with(tabled::Span::column(2)))
        .with(tabled::MinWidth::new(80))
        // .with(tabled::MaxWidth::wrapping(80))
    }};
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
        // .with(
        //     tabled::Modify::new(tabled::Columns::new(2..)).with(tabled::MaxWidth::wrapping(42)),
        // )
    };
}
