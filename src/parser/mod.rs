mod util;
pub use util::*;
mod common;
pub use common::*;
mod lexer;
pub use lexer::*;
mod node;
pub use node::*;
mod tags;
pub use tags::*;

use std::fmt::Display;

use chumsky::{prelude::Simple, Parser, Stream};

#[derive(Debug, Default)]
pub struct Rename {
    /// Prefix `function` name with `---@mod` name
    pub func: bool,
    /// Prefix `---@alias` tag with `return/---@mod` name
    pub alias: bool,
    /// Prefix ---@class tag with return/---@mod name
    pub class: bool,
    /// Prefix `---@type` tag with `---@mod` name
    pub r#type: bool,
}

#[derive(Debug, Default)]
pub struct LemmyHelp {
    rename: Rename,
    pub nodes: Vec<Node>,
}

impl LemmyHelp {
    pub fn with_rename(rename: Rename) -> Self {
        Self {
            rename,
            nodes: vec![],
        }
    }

    pub fn parse(&mut self, src: &str) -> Result<&Self, Vec<Simple<TagType>>> {
        self.nodes.append(&mut Self::lex(src)?);

        Ok(self)
    }

    /// Prepare nodes for help doc generation
    pub fn for_help(&mut self, src: &str) -> Result<&Self, Vec<Simple<TagType>>> {
        let mut nodes = Self::lex(src)?;

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

    fn lex(src: &str) -> Result<Vec<Node>, Vec<Simple<TagType>>> {
        let tokens = Lexer::parse(src).unwrap();
        let stream = Stream::from_iter(src.len()..src.len() + 1, tokens.into_iter());

        Node::parse().repeated().flatten().parse(stream)
    }
}

impl Display for LemmyHelp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            if let Node::Toc(x) = node {
                writeln!(
                    f,
                    "{}",
                    Module {
                        name: x.to_string(),
                        desc: Some("Table of Contents".into())
                    }
                )?;

                for nodde in &self.nodes {
                    if let Node::Module(x) = nodde {
                        let desc = x.desc.as_deref().unwrap_or_default();

                        writeln!(
                            f,
                            "{desc}{}",
                            format_args!("{:·>w$}", format!("|{}|", x.name), w = 80 - desc.len())
                        )?;
                    }
                }

                writeln!(f)?;
            } else {
                writeln!(f, "{node}")?;
            }
        }

        Ok(())
    }
}
