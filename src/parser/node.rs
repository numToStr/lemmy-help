use chumsky::{
    prelude::choice,
    primitive::{any, end},
    recovery::skip_then_retry_until,
    select, Parser,
};

use crate::{
    parser::{Alias, Brief, Class, Divider, Func, Module, Token, Type},
    Accept, Visitor,
};

use super::{
    alias_parser, brief_parser, class_parser, divider_parser, func_parser, mod_parser, tag_parser,
    type_parser, LemmyParser, Tag,
};

#[derive(Debug, Clone)]
pub enum Node<'src> {
    Module(Module<'src>),
    Divider(Divider),
    Brief(Brief<'src>),
    Tag(Tag<'src>),
    Func(Func<'src>),
    Class(Class<'src>),
    Alias(Alias<'src>),
    Type(Type<'src>),
    Export(&'src str),
    Toc(&'src str),
}

pub fn node_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Node<'src>> {
    choice((
        mod_parser(),
        divider_parser(),
        brief_parser(),
        tag_parser(),
        func_parser(),
        class_parser(),
        alias_parser(),
        type_parser(),
        select! {
            Token::Export(x) => Node::Export(x),
            Token::Toc(x) => Node::Toc(x),
        },
    ))
    .recover_with(skip_then_retry_until(any().ignored(), end()))
}

impl<'src, T: Visitor> Accept<T> for Node<'src> {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        match self {
            Self::Brief(x) => x.accept(n, s),
            Self::Tag(x) => x.accept(n, s),
            Self::Alias(x) => x.accept(n, s),
            Self::Func(x) => x.accept(n, s),
            Self::Class(x) => x.accept(n, s),
            Self::Type(x) => x.accept(n, s),
            Self::Module(x) => x.accept(n, s),
            Self::Divider(x) => x.accept(n, s),
            _ => unimplemented!(),
        }
    }
}

// impl Node<'_> {
//     /// Creates stream of AST nodes from emmylua
//     ///
//     /// ```
//     /// let src = r#"
//     /// local U = {}
//     ///
//     /// ---Add two integar and print it
//     /// ---@param this number First number
//     /// ---@param that number Second number
//     /// function U.sum(this, that)
//     ///     print(this + that)
//     /// end
//     ///
//     /// return U
//     /// "#;
//     ///
//     /// let nodes = lemmy_help::parser::Node::new(src).unwrap();
//     /// assert!(!nodes.is_empty());
//     /// ```
//     pub fn init<'src>(src: &'src str) -> Result<Vec<Node<'src>>, Vec<Rich<'src, Token<'src>>>> {
//         let tokens = Lexer::init().parse(src).into_output().unwrap().as_slice();
//         //     return Err(vec![])
//         // };
//         Node::parse()
//             .repeated()
//             .collect::<Vec<Node<'src>>>()
//             .parse(tokens.spanned((src.len()..src.len()).into()))
//             .into_result()
//     }
// }
