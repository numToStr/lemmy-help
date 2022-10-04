use std::fmt::Display;

use chumsky::{
    prelude::Simple,
    primitive::{choice, just},
    select,
    text::{ident, TextParser},
    Parser,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Primary {
    Nil,
    Any,
    Boolean,
    String,
    Number,
    Integer,
    Function,
    Table,
    Thread,
    Userdata,
    Lightuserdata,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arg(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TyToken {
    Primary(Primary),
    Arg(Arg),
    Fun,
    LeftAngle,
    RightAngle,
    LeftSquare,
    RightSquare,
    LeftBracket,
    RightBracket,
    Pipe,
    Comma,
    Colon,
}

// Source: https://github.com/sumneko/lua-language-server/wiki/Annotations#documenting-types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Primary(Primary),
    Union(Vec<Primary>), // TODO: union of static 'string' or number
    Array(Primary),
    Table(Option<(Primary, Primary)>),         // TODO: Box<Expr>
    Fun(Vec<(Arg, Primary)>, Option<Primary>), // TODO: Box<Expr>
}

impl Ty {
    pub fn parse() -> impl chumsky::Parser<char, Self, Error = Simple<char>> {
        let idnt = ident::<char, Simple<char>>().map(|id| match id.as_str() {
            "nil" => TyToken::Primary(Primary::Nil),
            "any" => TyToken::Primary(Primary::Any),
            "boolean" => TyToken::Primary(Primary::Boolean),
            "string" => TyToken::Primary(Primary::String),
            "number" => TyToken::Primary(Primary::Number),
            "integer" => TyToken::Primary(Primary::Integer),
            "function" => TyToken::Primary(Primary::Function),
            "table" => TyToken::Primary(Primary::Table),
            "thread" => TyToken::Primary(Primary::Thread),
            "userdata" => TyToken::Primary(Primary::Userdata),
            "lightuserdata" => TyToken::Primary(Primary::Lightuserdata),
            "fun" => TyToken::Fun,
            _ => TyToken::Arg(Arg(id)),
        });

        let ctrl = choice((
            just('<').to(TyToken::LeftAngle),
            just('>').to(TyToken::RightAngle),
            just('[').to(TyToken::LeftSquare),
            just(']').to(TyToken::RightSquare),
            just('(').to(TyToken::LeftBracket),
            just(')').to(TyToken::RightBracket),
            just('|').to(TyToken::Pipe),
            just(',').to(TyToken::Comma),
            just(':').to(TyToken::Colon),
        ));

        idnt.or(ctrl.padded()).repeated().collect().map(Ty::ast)
    }

    fn ast(tokens: Vec<TyToken>) -> Self {
        choice::<_, Simple<TyToken>>((
            just(TyToken::Fun)
                .ignore_then(
                    select! { TyToken::Arg(p) => p }
                        .then_ignore(just(TyToken::Colon))
                        .then(select! { TyToken::Primary(x) => x })
                        .separated_by(just(TyToken::Comma))
                        .allow_trailing()
                        .delimited_by(just(TyToken::LeftBracket), just(TyToken::RightBracket)),
                )
                .then(
                    just(TyToken::Colon)
                        .ignore_then(select! { TyToken::Primary(x) => x })
                        .or_not(),
                )
                .map(|(param, ret)| Ty::Fun(param, ret)),
            just(TyToken::Primary(Primary::Table))
                .ignore_then(
                    select! { TyToken::Primary(x) => x }
                        .then_ignore(just(TyToken::Comma))
                        .then(select! { TyToken::Primary(x) => x })
                        .or_not(),
                )
                .map(Ty::Table),
            select! { TyToken::Primary(x) => x }
                .separated_by(just(TyToken::Pipe))
                .map(Ty::Union),
            select! { TyToken::Primary(x) => x }
                .then_ignore(just(TyToken::LeftSquare))
                .then_ignore(just(TyToken::RightSquare))
                .map(Ty::Array),
            select! { TyToken::Primary(x) => x }.map(Ty::Primary),
        ))
        .parse(tokens)
        .unwrap()
    }
}

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ty-ty")
    }
}
