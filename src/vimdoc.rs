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
    FromEmmy, Settings,
};

use self::{
    alias::AliasDoc, brief::BriefDoc, class::ClassDoc, divider::DividerDoc, func::FuncDoc,
    module::ModuleDoc, r#type::TypeDoc, tag::TagDoc,
};

pub(crate) trait ToDoc {
    type N;
    fn to_doc(n: &Self::N, s: &Settings) -> String;
}

#[derive(Debug)]
pub struct VimDoc(String);

impl FromEmmy for VimDoc {
    type Settings = Settings;
    fn from_emmy(t: &impl crate::Nodes, s: &Self::Settings) -> Self {
        let mut doc = String::new();
        for node in t.nodes() {
            if let Node::Toc(x) = node {
                doc.push_str(
                    &ModuleDoc::to_doc(
                        &Module {
                            name: x.to_string(),
                            desc: Some("Table of Contents".into()),
                        },
                        s,
                    )
                    .to_string(),
                );
                doc.push('\n');

                for nod in t.nodes() {
                    if let Node::Module(x) = nod {
                        let desc = x.desc.as_deref().unwrap_or_default();

                        doc.push_str(&format!(
                            "{desc}{:·>w$}\n",
                            format!("|{}|", x.name),
                            w = 80 - desc.len()
                        ));
                    }
                }
            } else {
                let n = match node {
                    Node::Brief(x) => BriefDoc::to_doc(x, s),
                    Node::Tag(x) => TagDoc::to_doc(x, s),
                    Node::Alias(x) => AliasDoc::to_doc(x, s),
                    Node::Func(x) => FuncDoc::to_doc(x, s),
                    Node::Class(x) => ClassDoc::to_doc(x, s),
                    Node::Type(x) => TypeDoc::to_doc(x, s),
                    Node::Module(x) => ModuleDoc::to_doc(x, s),
                    Node::Divider(x) => DividerDoc::to_doc(x, s),
                    _ => unimplemented!(),
                };
                doc.push_str(&n);
            }

            doc.push('\n');
        }

        Self(doc)
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

#[inline]
pub(crate) fn description(desc: &str) -> String {
    let mut d = textwrap::indent(desc, "    ");
    d.push('\n');
    d
}

macro_rules! header {
    ($name:expr, $tag:expr) => {{
        let len = $name.len();
        if len > 40 || $tag.len() > 40 {
            format!("{:>80}\n", format!("*{}*", $tag));
            format!("{}\n", $name)
        } else {
            format!(
                "{}{}\n",
                $name,
                format_args!("{:>w$}", format!("*{}*", $tag), w = 80 - len)
            )
        }
    }};
    ($name:expr) => {
        super::header!($name, $name)
    };
}

pub(super) use header;
