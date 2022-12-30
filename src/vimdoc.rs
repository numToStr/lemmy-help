use std::fmt::Display;

use crate::{
    lexer::{Name, Scope},
    parser::{AliasKind, Divider, Module, Node},
    Accept, FromEmmy, Layout, Settings, Visitor,
};

#[derive(Debug)]
pub struct VimDoc(String);

impl Visitor for VimDoc {
    type R = String;
    type S = Settings;

    fn module(&self, n: &Module, s: &Self::S) -> Self::R {
        let mut doc = String::new();
        let desc = n.desc.as_deref().unwrap_or_default();
        doc.push_str(&self.divider(&Divider('='), s));
        doc.push_str(desc);
        doc.push_str(&format!(
            "{:>w$}",
            format!("*{}*", n.name),
            w = 80 - desc.len()
        ));
        doc.push('\n');
        doc
    }

    fn divider(&self, n: &crate::parser::Divider, _: &Self::S) -> Self::R {
        let mut doc = String::with_capacity(81);
        for _ in 0..80 {
            doc.push(n.0);
        }
        doc.push('\n');
        doc
    }

    fn brief(&self, n: &crate::parser::Brief, _: &Self::S) -> Self::R {
        let mut doc = n.desc.join("\n");
        doc.push('\n');
        doc
    }

    fn tag(&self, n: &crate::parser::Tag, _: &Self::S) -> Self::R {
        format!("{:>80}", format!("*{}*", n.0))
    }

    fn func(&self, n: &crate::parser::Func, s: &Self::S) -> Self::R {
        let mut doc = String::new();
        let name_with_param = if !n.params.is_empty() {
            let args = n
                .params
                .iter()
                .map(|p| format!("{{{}}}", p.name))
                .collect::<Vec<String>>()
                .join(", ");
            format!(
                "{}{}({args})",
                n.prefix.left.as_deref().unwrap_or_default(),
                n.op
            )
        } else {
            format!("{}{}()", n.prefix.left.as_deref().unwrap_or_default(), n.op)
        };
        doc.push_str(&header(
            &name_with_param,
            &format!("{}{}", n.prefix.right.as_deref().unwrap_or_default(), n.op),
        ));
        if !n.desc.is_empty() {
            doc.push_str(&description(&n.desc.join("\n")))
        }
        doc.push('\n');
        if !n.params.is_empty() {
            doc.push_str(&description("Parameters: ~"));
            doc.push_str(&self.params(&n.params, s));
            doc.push('\n');
        }
        if !n.returns.is_empty() {
            doc.push_str(&description("Returns: ~"));
            doc.push_str(&self.returns(&n.returns, s));
            doc.push('\n');
        }
        if !n.see.refs.is_empty() {
            doc.push_str(&self.see(&n.see, s));
        }
        if let Some(usage) = &n.usage {
            doc.push_str(&self.usage(usage, s));
        }
        doc
    }

    fn params(&self, n: &[crate::parser::Param], s: &Self::S) -> Self::R {
        let mut table = Table::new();
        for param in n {
            let (name, ty) = match (s.expand_opt, &param.name) {
                (true, Name::Opt(n)) => (format!("{{{n}}}"), format!("(nil|{})", param.ty)),
                (_, n) => (format!("{{{n}}}"), format!("({})", param.ty)),
            };
            match s.layout {
                Layout::Default => {
                    table.add_row([name, ty, param.desc.join("\n")]);
                }
                Layout::Compact(n) => {
                    table.add_row([
                        name,
                        format!(
                            "{ty} {}",
                            param.desc.join(&format!("\n{}", " ".repeat(n as usize)))
                        ),
                    ]);
                }
                Layout::Mini(n) => {
                    table.add_row([format!(
                        "{name} {ty} {}",
                        param.desc.join(&format!("\n{}", " ".repeat(n as usize)))
                    )]);
                }
            }
        }
        table.to_string()
    }

    fn r#returns(&self, n: &[crate::parser::Return], s: &Self::S) -> Self::R {
        let mut table = Table::new();
        for entry in n {
            if let Layout::Mini(n) = s.layout {
                table.add_row([format!(
                    "({}) {}",
                    entry.ty,
                    if entry.desc.is_empty() {
                        entry.name.clone().unwrap_or_default()
                    } else {
                        entry.desc.join(&format!("\n{}", " ".repeat(n as usize)))
                    }
                )]);
            } else {
                table.add_row([
                    format!("({})", entry.ty),
                    if entry.desc.is_empty() {
                        entry.name.clone().unwrap_or_default()
                    } else {
                        entry.desc.join("\n")
                    },
                ]);
            }
        }
        table.to_string()
    }

    fn class(&self, n: &crate::parser::Class, s: &Self::S) -> Self::R {
        let mut doc = String::new();
        let name = format!(
            "{}{}",
            n.name,
            n.parent
                .as_ref()
                .map_or(String::new(), |parent| format!(" : {parent}"))
        );
        if let Some(prefix) = &n.prefix.right {
            doc.push_str(&header(&name, &format!("{prefix}.{}", n.name)));
        } else {
            doc.push_str(&header(&name, &n.name));
        }
        if !n.desc.is_empty() {
            doc.push_str(&description(&n.desc.join("\n")));
        }
        doc.push('\n');
        if !n.fields.is_empty() {
            doc.push_str(&description("Fields: ~"));
            doc.push_str(&self.fields(&n.fields, s));
            doc.push('\n');
        }
        if !n.see.refs.is_empty() {
            doc.push_str(&self.see(&n.see, s));
        }
        doc
    }

    fn fields(&self, n: &[crate::parser::Field], s: &Self::S) -> Self::R {
        let mut table = Table::new();
        for field in n {
            let (name, ty) = match (s.expand_opt, &field.name) {
                (true, Name::Opt(n)) => (format!("{{{n}}}"), format!("(nil|{})", field.ty)),
                (_, n) => (format!("{{{n}}}"), format!("({})", field.ty)),
            };
            if field.scope == Scope::Public {
                match s.layout {
                    Layout::Default => {
                        table.add_row([name, ty, field.desc.join("\n")]);
                    }
                    Layout::Compact(n) => {
                        table.add_row([
                            name,
                            format!(
                                "{ty} {}",
                                field.desc.join(&format!("\n{}", " ".repeat(n as usize)))
                            ),
                        ]);
                    }
                    Layout::Mini(n) => {
                        table.add_row([format!(
                            "{name} {ty} {}",
                            field.desc.join(&format!("\n{}", " ".repeat(n as usize)))
                        )]);
                    }
                };
            }
        }
        table.to_string()
    }

    fn alias(&self, n: &crate::parser::Alias, _: &Self::S) -> Self::R {
        let mut doc = String::new();
        if let Some(prefix) = &n.prefix.right {
            doc.push_str(&header(&n.name, &format!("{prefix}.{}", n.name)));
        } else {
            doc.push_str(&header(&n.name, &n.name));
        }
        if !n.desc.is_empty() {
            doc.push_str(&description(&n.desc.join("\n")));
        }
        doc.push('\n');
        match &n.kind {
            AliasKind::Type(ty) => {
                doc.push_str(&description("Type: ~"));
                let ty = ty.to_string();
                doc.push_str(&format!("{:>w$}", ty, w = 8 + ty.len()));
                doc.push('\n');
            }
            AliasKind::Enum(variants) => {
                doc.push_str(&description("Variants: ~"));
                let mut table = Table::new();
                for (ty, desc) in variants {
                    table.add_row([&format!("({})", ty), desc.as_deref().unwrap_or_default()]);
                }
                doc.push_str(&table.to_string());
            }
        }
        doc.push('\n');
        doc
    }

    fn r#type(&self, n: &crate::parser::Type, s: &Self::S) -> Self::R {
        let mut doc = String::new();
        doc.push_str(&header(
            &format!("{}{}", n.prefix.left.as_deref().unwrap_or_default(), n.op),
            &format!("{}{}", n.prefix.right.as_deref().unwrap_or_default(), n.op),
        ));
        let (extract, desc) = &n.desc;
        if !extract.is_empty() {
            doc.push_str(&description(&extract.join("\n")));
        }
        doc.push('\n');
        doc.push_str(&description("Type: ~"));
        let mut table = Table::new();
        table.add_row([&format!("({})", n.ty), desc.as_deref().unwrap_or_default()]);
        doc.push_str(&table.to_string());
        doc.push('\n');
        if !n.see.refs.is_empty() {
            doc.push_str(&self.see(&n.see, s));
        }
        if let Some(usage) = &n.usage {
            doc.push_str(&self.usage(usage, s));
        }
        doc
    }

    fn see(&self, n: &crate::parser::See, _: &Self::S) -> Self::R {
        let mut doc = String::new();
        doc.push_str(&description("See: ~"));
        for s in &n.refs {
            doc.push_str(&format!("        |{s}|\n"));
        }
        doc.push('\n');
        doc
    }

    fn usage(&self, n: &crate::parser::Usage, _: &Self::S) -> Self::R {
        let mut doc = String::new();
        doc.push_str(&description("Usage: ~"));
        doc.push('>');
        doc.push_str(n.lang.as_deref().unwrap_or("lua"));
        doc.push('\n');
        doc.push_str(&textwrap::indent(&n.code, "        "));
        doc.push_str("\n<\n\n");
        doc
    }

    fn toc(&self, n: &str, nodes: &[Node], s: &Self::S) -> Self::R {
        let mut doc = String::new();
        let module = self.module(
            &Module {
                name: n.to_string(),
                desc: Some("Table of Contents".into()),
            },
            s,
        );
        doc.push_str(&module);
        doc.push('\n');
        for nod in nodes {
            if let Node::Module(x) = nod {
                let desc = x.desc.as_deref().unwrap_or_default();
                doc.push_str(&format!(
                    "{desc} {:·>w$}\n",
                    format!(" |{}|", x.name),
                    w = (TW - 1) - desc.len()
                ));
            }
        }
        doc
    }
}

impl FromEmmy for VimDoc {
    type Settings = Settings;
    fn from_emmy(t: &impl crate::Nodes, s: &Self::Settings) -> Self {
        let mut shelf = Self(String::new());
        let nodes = t.nodes();
        for node in nodes {
            if let Node::Toc(x) = node {
                shelf.0.push_str(&shelf.toc(x, nodes, s));
            } else {
                shelf.0.push_str(&node.accept(&shelf, s));
            }
            shelf.0.push('\n');
        }
        shelf
    }
}

impl Display for VimDoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

// #################

struct Table(comfy_table::Table);

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
        f.write_str(&textwrap::indent(&self.0.trim_fmt(), "       "))?;
        f.write_str("\n")
    }
}

#[inline]
fn description(desc: &str) -> String {
    let mut d = textwrap::indent(desc, "    ");
    d.push('\n');
    d
}

#[inline]
fn header(name: &str, tag: &str) -> String {
    let len = name.len();
    if len > 40 || tag.len() > 40 {
        return format!("{:>80}\n{}\n", format!("*{}*", tag), name);
    }
    format!("{}{:>w$}\n", name, format!("*{}*", tag), w = 80 - len)
}
