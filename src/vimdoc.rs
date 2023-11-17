use std::fmt::Display;

use crate::{
    lexer::{Name, Scope},
    parser::{AliasKind, Divider, Module, Node},
    Accept, FromEmmy, Layout, Settings, Visitor,
};

/// Text Width
const TW: usize = 80;

#[derive(Debug, Default)]
pub struct VimDoc<'src>(Vec<Node<'src>>);

impl Visitor for VimDoc<'_> {
    type R = String;
    type S = Settings;

    fn module(&self, n: &Module, s: &Self::S) -> Self::R {
        let mut doc = String::new();
        let desc = n.desc.unwrap_or_default();
        doc.push_str(&self.divider(&Divider('='), s));
        doc.push_str(desc);
        doc.push_str(&format!(
            "{:>w$}",
            format!("*{}*", n.name),
            w = TW - desc.len()
        ));
        doc.push('\n');
        doc
    }

    fn divider(&self, n: &crate::parser::Divider, _: &Self::S) -> Self::R {
        let mut doc = String::with_capacity(TW - 1);
        for _ in 0..TW - 2 {
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
        format!("{:>w$}", format!("*{}*", n.0), w = TW)
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
                n.prefix.left.unwrap_or_default(),
                n.op.iter().map(|x| x.to_string()).collect::<String>()
            )
        } else {
            format!(
                "{}{}()",
                n.prefix.left.unwrap_or_default(),
                n.op.iter().map(|x| x.to_string()).collect::<String>()
            )
        };
        doc.push_str(&header(
            &name_with_param,
            &format!(
                "{}{}",
                n.prefix.right.unwrap_or_default(),
                n.op.iter().map(|x| x.to_string()).collect::<String>()
            ),
        ));
        if !n.desc.is_empty() {
            doc.push_str(&description(&n.desc.join("\n"), s.indent_width))
        }
        doc.push('\n');
        if !n.params.is_empty() {
            doc.push_str(&description("Parameters: ~", s.indent_width));
            doc.push_str(&self.params(&n.params, s));
            doc.push('\n');
        }
        if !n.returns.is_empty() {
            doc.push_str(&description("Returns: ~", s.indent_width));
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
        let mut table = Table::new(s.indent_width);
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
        let mut table = Table::new(s.indent_width);
        for entry in n {
            if let Layout::Mini(n) = s.layout {
                table.add_row([format!(
                    "({}) {}",
                    entry.ty,
                    if entry.desc.is_empty() {
                        entry.name.unwrap_or_default().to_owned()
                    } else {
                        entry.desc.join(&format!("\n{}", " ".repeat(n as usize)))
                    }
                )]);
            } else {
                table.add_row([
                    format!("({})", entry.ty),
                    if entry.desc.is_empty() {
                        entry.name.unwrap_or_default().to_owned()
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
            doc.push_str(&header(&name, n.name));
        }
        if !n.desc.is_empty() {
            doc.push_str(&description(&n.desc.join("\n"), s.indent_width));
        }
        doc.push('\n');
        if !n.fields.is_empty() {
            doc.push_str(&description("Fields: ~", s.indent_width));
            doc.push_str(&self.fields(&n.fields, s));
            doc.push('\n');
        }
        if !n.see.refs.is_empty() {
            doc.push_str(&self.see(&n.see, s));
        }
        doc
    }

    fn fields(&self, n: &[crate::parser::Field], s: &Self::S) -> Self::R {
        let mut table = Table::new(s.indent_width);
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

    fn alias(&self, n: &crate::parser::Alias, s: &Self::S) -> Self::R {
        let mut doc = String::new();
        if let Some(prefix) = &n.prefix.right {
            doc.push_str(&header(n.name, &format!("{prefix}.{}", n.name)));
        } else {
            doc.push_str(&header(n.name, n.name));
        }
        if !n.desc.is_empty() {
            doc.push_str(&description(&n.desc.join("\n"), s.indent_width));
        }
        doc.push('\n');
        match &n.kind {
            AliasKind::Type(ty) => {
                doc.push_str(&description("Type: ~", s.indent_width));
                doc.push_str(&(" ").repeat(s.indent_width * 2));
                doc.push_str(&ty.to_string());
                doc.push('\n');
            }
            AliasKind::Enum(variants) => {
                doc.push_str(&description("Variants: ~", s.indent_width));
                let mut table = Table::new(s.indent_width);
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
            &format!(
                "{}{}",
                n.prefix.left.unwrap_or_default(),
                n.op.iter().map(|x| x.to_string()).collect::<String>()
            ),
            &format!(
                "{}{}",
                n.prefix.right.unwrap_or_default(),
                n.op.iter().map(|x| x.to_string()).collect::<String>()
            ),
        ));
        let (extract, desc) = &n.desc;
        if !extract.is_empty() {
            doc.push_str(&description(&extract.join("\n"), s.indent_width));
        }
        doc.push('\n');
        doc.push_str(&description("Type: ~", s.indent_width));
        let mut table = Table::new(s.indent_width);
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

    fn see(&self, n: &crate::parser::See, s: &Self::S) -> Self::R {
        let mut doc = String::new();
        doc.push_str(&description("See: ~", s.indent_width));
        for reff in &n.refs {
            doc.push_str(&(" ").repeat(s.indent_width * 2));
            doc.push_str(&format!("|{reff}|\n"));
        }
        doc.push('\n');
        doc
    }

    fn usage(&self, n: &crate::parser::Usage, s: &Self::S) -> Self::R {
        let mut doc = String::new();
        doc.push_str(&description("Usage: ~", s.indent_width));
        doc.push('>');
        doc.push_str(n.lang.unwrap_or("lua"));
        doc.push('\n');
        doc.push_str(&textwrap::indent(
            &n.code.to_string(),
            &(" ").repeat(s.indent_width * 2),
        ));
        doc.push_str("\n<\n\n");
        doc
    }

    fn toc(&self, n: &str, nodes: &[Node], s: &Self::S) -> Self::R {
        let mut doc = String::new();
        let module = self.module(
            &Module {
                name: n,
                desc: Some("Table of Contents"),
            },
            s,
        );
        doc.push_str(&module);
        doc.push('\n');
        for nod in nodes {
            if let Node::Module(x) = nod {
                let desc = x.desc.unwrap_or_default();
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

impl<'src> FromEmmy<'src> for VimDoc<'src> {
    type Settings = Settings;
    fn from_emmy(t: &'src impl crate::Nodes<'src>, setting: &Self::Settings) -> Self {
        let mut emmynodes = t.nodes();

        let Some(Node::Export(export)) = emmynodes.pop() else {
            return Self::default()
        };

        let mut nodes = vec![];

        let module = match emmynodes
            .iter()
            .rev()
            .find(|x| matches!(x, Node::Module(_)))
        {
            Some(Node::Module(m)) => m.name,
            _ => export,
        };

        for ele in emmynodes {
            match ele {
                Node::Export(..) => {}
                Node::Func(mut func) => {
                    if func.prefix.left == Some(export) {
                        if setting.prefix_func {
                            func.prefix.right = Some(module);
                        }
                        nodes.push(Node::Func(func));
                    }
                }
                Node::Type(mut typ) => {
                    if typ.prefix.left == Some(export) {
                        if setting.prefix_type {
                            typ.prefix.right = Some(module);
                        }
                        nodes.push(Node::Type(typ));
                    }
                }
                Node::Alias(mut alias) => {
                    if setting.prefix_alias {
                        alias.prefix.right = Some(module);
                    }
                    nodes.push(Node::Alias(alias))
                }
                Node::Class(mut class) => {
                    if setting.prefix_class {
                        class.prefix.right = Some(module);
                    }
                    nodes.push(Node::Class(class))
                }
                x => nodes.push(x),
            }
        }

        // let mut shelf = String::new();
        // let nodes = t.nodes();
        // for node in nodes {
        //     if let Node::Toc(x) = node {
        //         shelf.push_str(&shelf.toc(x, nodes, s));
        //     } else {
        //         shelf.push_str(&node.accept(&shelf, s));
        //     }
        //     shelf.push('\n');
        // }
        // shelf

        Self(nodes)
    }
}

impl Display for VimDoc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// #################

struct Table(comfy_table::Table, usize);

impl Table {
    pub fn new(ident: usize) -> Self {
        let mut tbl = comfy_table::Table::new();
        tbl.load_preset(comfy_table::presets::NOTHING);
        // tbl.column_iter_mut().map(|c| c.set_padding((0, 0)));
        Self(tbl, ident)
    }

    pub fn add_row<T: Into<comfy_table::Row>>(&mut self, row: T) -> &Self {
        self.0.add_row(row);
        self
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&textwrap::indent(
            &self.0.trim_fmt(),
            &(" ").repeat((self.1 * 2) - 1),
        ))?;
        f.write_str("\n")
    }
}

#[inline]
fn description(desc: &str, indent: usize) -> String {
    let mut d = textwrap::indent(desc, &(" ").repeat(indent));
    d.push('\n');
    d
}

#[inline]
fn header(name: &str, tag: &str) -> String {
    let len = name.len();
    if len > 40 || tag.len() > 40 {
        return format!("{:>w$}\n{}\n", format!("*{}*", tag), name, w = TW);
    }
    format!("{}{:>w$}\n", name, format!("*{}*", tag), w = TW - len)
}
