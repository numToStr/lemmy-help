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

use chumsky::{prelude::Simple, Parser, Stream};

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
pub struct LemmyHelp {
    pub nodes: Vec<Node>,
}

impl LemmyHelp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(&mut self, src: &str) -> Result<&Self, Vec<Simple<TagType>>> {
        self.nodes.append(&mut Self::lex(src)?);

        Ok(self)
    }

    /// Prepare nodes for help doc generation
    pub fn for_help(&mut self, src: &str) -> Result<&Self, Vec<Simple<TagType>>> {
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
                    _ => self.nodes.push(ele),
                }
            }
        };

        Ok(self)
    }

    fn lex(src: &str) -> Result<Vec<Node>, Vec<Simple<TagType>>> {
        let tokens = Emmy::parse(src).unwrap();
        let stream = Stream::from_iter(src.len()..src.len() + 1, tokens.into_iter());

        Node::parse().repeated().flatten().parse(stream)
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
