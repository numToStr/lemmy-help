use chumsky::{select, IterParser, Parser};

use crate::{
    lexer::{Name, Scope, Token, Ty},
    parser::{LemmyParser, Node, Prefix, See},
    Accept, Visitor,
};

use super::see_parser;

#[derive(Debug, Clone)]
pub struct Field<'src> {
    pub scope: Scope,
    pub name: Name<'src>,
    pub ty: Ty<'src>,
    pub desc: Vec<&'src str>,
}

pub fn field_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Field<'src>> {
    select! { Token::Comment(x) => x }
        .repeated()
        .collect::<Vec<&'src str>>()
        .then(select! { Token::Field(scope,name,ty,desc) => (scope,name,ty,desc) })
        .map(|(header, (scope, name, ty, desc))| {
            let desc = match desc {
                Some(d) => {
                    let mut new_desc = Vec::with_capacity(header.len() + 1);
                    new_desc.push(d);
                    new_desc.extend(header);
                    new_desc
                }
                None => header,
            };
            Field {
                scope,
                name,
                ty,
                desc,
            }
        })
}

#[derive(Debug, Clone)]
pub struct Class<'src> {
    pub name: &'src str,
    pub parent: Option<&'src str>,
    pub desc: Vec<&'src str>,
    pub fields: Vec<Field<'src>>,
    pub see: See<'src>,
    pub prefix: Prefix<'src>,
}

pub fn class_parser<'tokens, 'src: 'tokens>() -> impl LemmyParser<'tokens, 'src, Node<'src>> {
    select! { Token::Comment(c) => c }
        .repeated()
        .collect()
        .then(select! { Token::Class(name, parent) => (name,parent) })
        .then(field_parser().repeated().collect())
        .then(see_parser())
        .map(|(((desc, (name, parent)), fields), see)| {
            Node::Class(Class {
                name,
                parent,
                desc,
                fields,
                see,
                prefix: Prefix::default(),
            })
        })
}

impl<'src, T: Visitor> Accept<T> for Class<'src> {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.class(self, s)
    }
}
