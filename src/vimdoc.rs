mod alias;
mod brief;
mod class;
mod divider;
mod func;
mod module;
mod see;
mod tag;
mod r#type;
mod usage;

use std::fmt::Display;

use crate::{
    parser::{Module, Node},
    FromEmmy,
};

#[derive(Debug)]
pub struct VimDoc(String);

impl FromEmmy for VimDoc {
    type Settings = ();
    fn from_emmy(t: &impl crate::Nodes, _: Self::Settings) -> Self {
        let mut doc = Vec::new();
        for node in t.nodes() {
            if let Node::Toc(x) = node {
                doc.push(
                    module::ModuleDoc(&Module {
                        name: x.to_string(),
                        desc: Some("Table of Contents".into()),
                    })
                    .to_string(),
                );
                doc.push("\n".to_string());

                for nod in t.nodes() {
                    if let Node::Module(x) = nod {
                        let desc = x.desc.as_deref().unwrap_or_default();

                        doc.push(format!(
                            "{desc}{:·>w$}\n",
                            format!("|{}|", x.name),
                            w = 80 - desc.len()
                        ));
                    }
                }
            } else {
                let n = match node {
                    Node::Brief(x) => brief::BriefDoc(x).to_string(),
                    Node::Tag(x) => tag::TagDoc(x).to_string(),
                    Node::Alias(x) => alias::AliasDoc(x).to_string(),
                    Node::Func(x) => func::FuncDoc(x).to_string(),
                    Node::Class(x) => class::ClassDoc(x).to_string(),
                    Node::Type(x) => r#type::TypeDoc(x).to_string(),
                    Node::Module(x) => module::ModuleDoc(x).to_string(),
                    Node::Divider(x) => divider::DividerDoc(x).to_string(),
                    _ => unimplemented!(),
                };
                doc.push(n);
            }

            doc.push("\n".to_string());
        }

        Self(doc.into_iter().collect())
    }
}

impl Display for VimDoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

// #################

pub(crate) struct Table(comfy_table::Table);

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
        super::header!($f, $name, $name)
    };
}

macro_rules! description {
    ($f:expr, $desc:expr) => {
        writeln!($f, "{}", textwrap::indent($desc, "    "))
    };
}

pub(super) use description;
pub(super) use header;
