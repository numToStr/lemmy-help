use std::fmt::Display;

use chumsky::{
    prelude::Simple,
    primitive::{choice, just},
    recursive::recursive,
    text::{ident, TextParser},
    Parser,
};

// Source: https://github.com/sumneko/lua-language-server/wiki/Annotations#documenting-types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Nil,
    Unknown,
    Boolean,
    String,
    Number,
    Integer,
    Function,
    Thread,
    Userdata,
    Lightuserdata,
    Union(Vec<Ty>), // TODO: 1) Box<Ty> 2) union of static 'string' or number
    Array(Box<Ty>), // TODO: Box<Ty>
    Table(Option<(Box<Ty>, Box<Ty>)>),
    Fun(Vec<(String, Ty)>, Option<Box<Ty>>),
}

impl Ty {
    pub fn parse() -> impl chumsky::Parser<char, Self, Error = Simple<char>> {
        recursive(|inner| {
            let comma = just(',').padded();
            let colon = just(':').padded();

            // let primary = choice((
            //     just("any").to(Ty::Unknown),
            //     just("unknown").to(Ty::Unknown),
            //     just("nil").to(Ty::Nil),
            //     just("boolean").to(Ty::Boolean),
            //     just("string").to(Ty::String),
            //     just("number").to(Ty::Number),
            //     just("integer").to(Ty::Integer),
            //     just("function").to(Ty::Function),
            //     just("thread").to(Ty::Thread),
            //     just("userdata").to(Ty::Userdata),
            //     just("lightuserdata").to(Ty::Lightuserdata),
            // ));

            choice((
                just("any").to(Ty::Unknown),
                just("unknown").to(Ty::Unknown),
                just("nil").to(Ty::Nil),
                just("boolean").to(Ty::Boolean),
                just("string").to(Ty::String),
                just("number").to(Ty::Number),
                just("integer").to(Ty::Integer),
                just("function").to(Ty::Function),
                just("thread").to(Ty::Thread),
                just("userdata").to(Ty::Userdata),
                just("lightuserdata").to(Ty::Lightuserdata),
                just("fun")
                    .ignore_then(
                        ident()
                            .padded()
                            .then_ignore(colon)
                            .then(inner.clone())
                            .separated_by(comma)
                            .allow_trailing()
                            .delimited_by(just('('), just(')')),
                    )
                    .then(colon.ignore_then(inner.clone().map(Box::new)).or_not())
                    .map(|(param, ret)| Ty::Fun(param, ret)),
                just("table")
                    .ignore_then(
                        just('<')
                            .ignore_then(inner.clone().map(Box::new))
                            .then_ignore(comma)
                            .then(inner.clone().map(Box::new))
                            .then_ignore(just('>'))
                            .or_not(),
                    )
                    .map(Ty::Table),
                // inner
                //     .clone()
                //     .then_ignore(just('|'))
                //     .chain(inner.clone().separated_by(just('|')))
                //     .map(Ty::Union),
                // inner
                //     .then_ignore(just('[').then(just(']')))
                //     .map(|x| Ty::Array(Box::new(x))),
            ))
        })
    }
}

#[test]
fn ty_parse() {
    let conds = [
        "fun(a: string, b: string, c: function, d: fun(z: string)): table<string, string>",
        "table<string, fun(a: string): string>",
        "table<fun(), table<string, number>>",
        "table<string, fun(a: string, b: table<string, boolean>)>",
        // Not working
        "fun(a: string, b: string|number, c: string[], d: number[][])",
        "table<string, string|number>[]",
    ];

    for t in conds {
        let a = Ty::parse().parse(t).unwrap();
        dbg!(a);
    }
}

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ty-ty")
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// enum TyToken {
//     Primary(Primary),
//     Arg(Arg),
//     Fun,
//     LeftAngle,
//     RightAngle,
//     LeftSquare,
//     RightSquare,
//     LeftBracket,
//     RightBracket,
//     Pipe,
//     Comma,
//     Colon,
// }
//
// impl Ty {
//     pub fn parse() -> impl chumsky::Parser<char, Self, Error = Simple<char>> {
//         let idnt = ident::<char, Simple<char>>().map(|id| match id.as_str() {
//             "nil" => TyToken::Primary(Primary::Nil),
//             "any" => TyToken::Primary(Primary::Any),
//             "boolean" => TyToken::Primary(Primary::Boolean),
//             "string" => TyToken::Primary(Primary::String),
//             "number" => TyToken::Primary(Primary::Number),
//             "integer" => TyToken::Primary(Primary::Integer),
//             "function" => TyToken::Primary(Primary::Function),
//             "table" => TyToken::Primary(Primary::Table),
//             "thread" => TyToken::Primary(Primary::Thread),
//             "userdata" => TyToken::Primary(Primary::Userdata),
//             "lightuserdata" => TyToken::Primary(Primary::Lightuserdata),
//             "fun" => TyToken::Fun,
//             _ => TyToken::Arg(Arg(id)),
//         });
//
//         let ctrl = choice((
//             just('<').to(TyToken::LeftAngle),
//             just('>').to(TyToken::RightAngle),
//             just('[').to(TyToken::LeftSquare),
//             just(']').to(TyToken::RightSquare),
//             just('(').to(TyToken::LeftBracket),
//             just(')').to(TyToken::RightBracket),
//             just('|').to(TyToken::Pipe),
//             just(',').to(TyToken::Comma),
//             just(':').to(TyToken::Colon),
//         ));
//
//         idnt.or(ctrl.padded()).repeated().collect().map(Ty::ast)
//     }
//
//     fn ast(tokens: Vec<TyToken>) -> Self {
//         choice::<_, Simple<TyToken>>((
//             just(TyToken::Fun)
//                 .ignore_then(
//                     select! { TyToken::Arg(p) => p }
//                         .then_ignore(just(TyToken::Colon))
//                         .then(select! { TyToken::Primary(x) => x })
//                         .separated_by(just(TyToken::Comma))
//                         .allow_trailing()
//                         .delimited_by(just(TyToken::LeftBracket), just(TyToken::RightBracket)),
//                 )
//                 .then(
//                     just(TyToken::Colon)
//                         .ignore_then(select! { TyToken::Primary(x) => x })
//                         .or_not(),
//                 )
//                 .map(|(param, ret)| Ty::Fun(param, ret)),
//             just(TyToken::Primary(Primary::Table))
//                 .ignore_then(
//                     select! { TyToken::Primary(x) => x }
//                         .then_ignore(just(TyToken::Comma))
//                         .then(select! { TyToken::Primary(x) => x })
//                         .or_not(),
//                 )
//                 .map(Ty::Table),
//             select! { TyToken::Primary(x) => x }
//                 .separated_by(just(TyToken::Pipe))
//                 .map(Ty::Union),
//             select! { TyToken::Primary(x) => x }
//                 .then_ignore(just(TyToken::LeftSquare))
//                 .then_ignore(just(TyToken::RightSquare))
//                 .map(Ty::Array),
//             select! { TyToken::Primary(x) => x }.map(Ty::Primary),
//         ))
//         .parse(tokens)
//         .unwrap()
//     }
// }
