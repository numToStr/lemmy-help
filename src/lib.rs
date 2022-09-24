#[cfg(feature = "vimdoc")]
pub mod vimdoc;

pub mod lexer;
pub mod parser;

use std::fmt::Display;

use chumsky::prelude::Simple;
use parser::Node;

use crate::lexer::TagType;

pub trait Nodes {
    fn nodes(&self) -> &Vec<Node>;
}

pub trait FromEmmy: Display {
    type Settings;
    fn from_emmy(t: &impl Nodes, s: Self::Settings) -> Self;
}

pub trait AsDoc<T: FromEmmy> {
    fn as_doc(&self, s: T::Settings) -> T;
}

#[derive(Debug, Default)]
pub struct Rename {
    /// Prefix `function` name with `---@mod` name
    pub func: bool,
    /// Prefix `---@alias` tag with `---@mod/return` name
    pub alias: bool,
    /// Prefix `---@class` tag with `---@mod/return` name
    pub class: bool,
    /// Prefix `---@type` tag with `---@mod` name
    pub r#type: bool,
}

#[derive(Debug, Default)]
pub struct LemmyHelp {
    rename: Rename,
    nodes: Vec<Node>,
}

impl Nodes for LemmyHelp {
    fn nodes(&self) -> &Vec<Node> {
        &self.nodes
    }
}

impl<T: FromEmmy> AsDoc<T> for LemmyHelp {
    fn as_doc(&self, s: T::Settings) -> T {
        T::from_emmy(self, s)
    }
}

impl LemmyHelp {
    /// Creates a new parser instance
    ///
    /// ```
    /// use lemmy_help::{LemmyHelp, Rename};
    ///
    /// LemmyHelp::new(Rename::default());
    /// ```
    pub fn new(rename: Rename) -> Self {
        Self {
            rename,
            nodes: vec![],
        }
    }

    /// Parse given lua source code to generate AST representation
    ///
    /// ```
    /// use lemmy_help::{LemmyHelp, Nodes};
    ///
    /// let mut lemmy = LemmyHelp::default();
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
    ///
    /// let ast = lemmy.parse(&src).unwrap();
    /// assert!(!ast.nodes().is_empty());
    /// ```
    pub fn parse(&mut self, src: &str) -> Result<&Self, Vec<Simple<TagType>>> {
        self.nodes.append(&mut Node::new(src)?);

        Ok(self)
    }

    /// Similar to [`LemmyHelp::parse`], but specifically used for generating vimdoc
    pub fn for_help(&mut self, src: &str) -> Result<&Self, Vec<Simple<TagType>>> {
        let mut nodes = Node::new(src)?;

        if let Some(Node::Export(export)) = nodes.pop() {
            let module = match nodes.iter().rev().find(|x| matches!(x, Node::Module(_))) {
                Some(Node::Module(m)) => m.name.to_owned(),
                _ => export.to_owned(),
            };

            for ele in nodes {
                match ele {
                    Node::Export(..) => {}
                    Node::Func(mut func) => {
                        if func.is_public(&export) {
                            if self.rename.func {
                                func.rename_tag(module.to_owned());
                            }
                            self.nodes.push(Node::Func(func));
                        }
                    }
                    Node::Type(mut typ) => {
                        if typ.is_public(&export) {
                            if self.rename.r#type {
                                typ.rename_tag(module.to_owned());
                            }
                            self.nodes.push(Node::Type(typ));
                        }
                    }
                    Node::Alias(mut alias) => {
                        if self.rename.alias {
                            alias.rename_tag(module.to_owned());
                        }
                        self.nodes.push(Node::Alias(alias))
                    }
                    Node::Class(mut class) => {
                        if self.rename.class {
                            class.rename_tag(module.to_owned());
                        }
                        self.nodes.push(Node::Class(class))
                    }
                    _ => self.nodes.push(ele),
                }
            }
        };

        Ok(self)
    }
}
