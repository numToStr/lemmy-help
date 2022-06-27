mod util;

mod common;
pub use common::*;
mod emmy;
pub use emmy::*;
mod node;
pub use node::*;
mod tags;
pub use tags::*;

use std::fmt::Display;

use chumsky::{Parser, Stream};

use crate::{LemmyError, LemmyResult};

// A TYPE could be
// - primary = string|number|boolean
// - fn = func(...):string
// - enum = "one"|"two"|"three"
// - or: primary (| primary)+
// - optional = primary?
// - table = table<string, string>
// - array = primary[]

// ---@tag @comment

// ---@field [public|protected|private] field_name FIELD_TYPE[|OTHER_TYPE] [@comment]

// ---@param param_name MY_TYPE[|other_type] [@comment]

// ---@type MY_TYPE[|OTHER_TYPE] [@comment]

// ---@alias NEW_NAME TYPE [@comment]

// ---@see @comment

// ---@return MY_TYPE[|OTHER_TYPE] [@comment]

#[derive(Debug, Default)]
pub struct Rename {
    pub alias: bool,
    pub class: bool,
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

    pub fn parse(&mut self, src: &str) -> LemmyResult<&Self> {
        self.nodes.append(&mut Self::lex(src)?);

        Ok(self)
    }

    /// Prepare nodes for help doc generation
    pub fn for_help(&mut self, src: &str) -> LemmyResult<&Self> {
        let mut nodes = Self::lex(src)?;

        if let Some(Node::Export(export)) = nodes.pop() {
            let module = if let Some(Node::Module(Module { name, .. })) = nodes.first().cloned() {
                name
            } else {
                export.to_owned()
            };

            for ele in nodes {
                match ele {
                    Node::Export(..) => {}
                    Node::Func(mut func) => {
                        if func.is_public(&export) {
                            func.rename_tag(module.to_owned());

                            self.nodes.push(Node::Func(func));
                        }
                    }
                    Node::Type(mut typ) => {
                        if typ.is_public(&export) {
                            typ.rename_tag(module.to_owned());

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

    fn lex(src: &str) -> LemmyResult<Vec<Node>> {
        let tokens = Emmy::parse(src).map_err(LemmyError::Lexer)?;
        let stream = Stream::from_iter(src.len()..src.len() + 1, tokens.into_iter());

        Node::parse()
            .repeated()
            .flatten()
            .parse(stream)
            .map_err(LemmyError::Parser)
    }
}

impl Display for LemmyHelp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            writeln!(f, "{}", node)?;
        }

        write!(f, "")
    }
}
