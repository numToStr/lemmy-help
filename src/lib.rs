#[cfg(feature = "vimdoc")]
pub mod vimdoc;

pub mod lexer;
pub mod parser;

use std::{fmt::Display, str::FromStr};

use chumsky::{prelude::Input, IterParser, Parser};
use parser::{
    Alias, Brief, Class, Divider, Field, Func, Module, Node, Param, Return, See, Tag, Type, Usage,
};

use crate::{lexer::lexer, parser::node_parser};

pub trait Visitor {
    type R;
    type S;
    fn module(&self, n: &Module, s: &Self::S) -> Self::R;
    fn divider(&self, n: &Divider, s: &Self::S) -> Self::R;
    fn brief(&self, n: &Brief, s: &Self::S) -> Self::R;
    fn tag(&self, n: &Tag, s: &Self::S) -> Self::R;
    fn func(&self, n: &Func, s: &Self::S) -> Self::R;
    fn params(&self, n: &[Param], s: &Self::S) -> Self::R;
    fn r#returns(&self, n: &[Return], s: &Self::S) -> Self::R;
    fn class(&self, n: &Class, s: &Self::S) -> Self::R;
    fn fields(&self, n: &[Field], s: &Self::S) -> Self::R;
    fn alias(&self, n: &Alias, s: &Self::S) -> Self::R;
    fn r#type(&self, n: &Type, s: &Self::S) -> Self::R;
    fn toc(&self, n: &str, nodes: &[Node], s: &Self::S) -> Self::R;
    fn see(&self, n: &See, s: &Self::S) -> Self::R;
    fn usage(&self, n: &Usage, s: &Self::S) -> Self::R;
}

pub trait Accept<T: Visitor> {
    fn accept(&self, n: &T, s: &T::S) -> T::R;
}

pub trait Nodes<'src> {
    fn nodes(&'src self) -> &'src [Node<'src>];
    fn into_nodes(self) -> Vec<Node<'src>>;
}

pub trait FromEmmy<'src>: Display {
    type Settings;
    fn from_emmy(t: &'src impl Nodes<'src>, s: &Self::Settings) -> Self;
}

pub trait AsDoc<'src, T: FromEmmy<'src>> {
    fn as_doc(&'src self, s: &T::Settings) -> T;
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Layout {
    #[default]
    Default,
    Compact(u8),
    Mini(u8),
}

impl FromStr for Layout {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            x => {
                let mut val = x.splitn(2, ':');
                match (val.next(), val.next()) {
                    (Some("compact"), n) => Ok(Self::Compact(
                        n.map_or(0, |x| x.parse().unwrap_or_default()),
                    )),
                    (Some("mini"), n) => {
                        Ok(Self::Mini(n.map_or(0, |x| x.parse().unwrap_or_default())))
                    }
                    _ => Err(()),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Settings {
    /// Prefix `function` name with `---@mod` name
    pub prefix_func: bool,
    /// Prefix `---@alias` tag with `---@mod/return` name
    pub prefix_alias: bool,
    /// Prefix `---@class` tag with `---@mod/return` name
    pub prefix_class: bool,
    /// Prefix `---@type` tag with `---@mod` name
    pub prefix_type: bool,
    /// Expand `?` to `nil|<type>`
    pub expand_opt: bool,
    /// Vimdoc text layout
    pub layout: Layout,
    /// Controls the indent width
    pub indent_width: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            prefix_func: false,
            prefix_alias: false,
            prefix_class: false,
            prefix_type: false,
            expand_opt: false,
            layout: Layout::default(),
            indent_width: 4,
        }
    }
}

#[derive(Debug, Default)]
pub struct Document<'src> {
    nodes: Vec<Node<'src>>,
}

impl<'src> Document<'src> {
    pub fn new(nodes: Vec<Node<'src>>) -> Self {
        Self { nodes }
    }
}

impl<'src> Nodes<'src> for Document<'src> {
    fn nodes(&'src self) -> &[Node<'src>] {
        self.nodes.as_ref()
    }

    fn into_nodes(self) -> Vec<Node<'src>> {
        self.nodes
    }
}

impl<'src, T: FromEmmy<'src>> AsDoc<'src, T> for Document<'src> {
    fn as_doc(&'src self, s: &T::Settings) -> T {
        T::from_emmy(self, s)
    }
}

/// Parse given lua source code to generate AST representation
///
/// ```
/// use lemmy_help::{parser, Nodes, Settings};
///
/// let src = r#"
/// local U = {}
///
/// ---Add two integar and print it
/// ---@param this number First number
/// ---@param that number Second number
/// function U.sum(this, that)
///     print(this + that)
/// end
///
/// return U
/// "#;
/// let settings = Settings::default();
/// let ast = lemmy_help::parser(&src, &settings);
/// assert!(!ast.nodes().is_empty());
/// ```
pub fn parser<'src>(src: &'src str, settings: &'src Settings) -> Document<'src> {
    let Some(tokens) = lexer().parse(src).into_output() else {
        return Document::default()
    };

    let Some(mut emmynode) = node_parser()
            .repeated()
            .collect::<Vec<_>>()
            .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
            .into_output()
    else {
        return Document::default()
    };

    let Some(Node::Export(export)) = emmynode.pop() else {
        return Document::default()
    };

    let mut nodes = vec![];

    let module = match emmynode.iter().rev().find(|x| matches!(x, Node::Module(_))) {
        Some(Node::Module(m)) => m.name,
        _ => export,
    };

    for ele in emmynode {
        match ele {
            Node::Export(..) => {}
            Node::Func(mut func) => {
                if func.prefix.left == Some(export) {
                    if settings.prefix_func {
                        func.prefix.right = Some(module);
                    }
                    nodes.push(Node::Func(func));
                }
            }
            Node::Type(mut typ) => {
                if typ.prefix.left == Some(export) {
                    if settings.prefix_type {
                        typ.prefix.right = Some(module);
                    }
                    nodes.push(Node::Type(typ));
                }
            }
            Node::Alias(mut alias) => {
                if settings.prefix_alias {
                    alias.prefix.right = Some(module);
                }
                nodes.push(Node::Alias(alias))
            }
            Node::Class(mut class) => {
                if settings.prefix_class {
                    class.prefix.right = Some(module);
                }
                nodes.push(Node::Class(class))
            }
            x => nodes.push(x),
        }
    }

    Document { nodes }
}
