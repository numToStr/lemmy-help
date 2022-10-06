use std::fmt::Display;

use chumsky::{
    prelude::Simple,
    primitive::{choice, just},
    recursive::recursive,
    text::{ident, whitespace, TextParser},
    Parser,
};

// Source: https://github.com/sumneko/lua-language-server/wiki/Annotations#documenting-types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Nil,
    Any,
    Unknown,
    Boolean,
    String,
    Number,
    Integer,
    Function,
    Thread,
    Userdata,
    Lightuserdata,
    Union(Box<Ty>, Box<Ty>), // TODO: 2) union of static 'string' or number
    Array(Box<Ty>),
    Table(Option<(Box<Ty>, Box<Ty>)>),
    Fun(Vec<(String, Ty)>, Option<Box<Ty>>),
    Dict(Vec<(String, Ty)>),
}

impl Ty {
    pub fn parse() -> impl chumsky::Parser<char, Self, Error = Simple<char>> {
        recursive(|inner| {
            let comma = just(',').padded();
            let colon = just(':').padded();

            let any = just("any").to(Ty::Any);
            let unknown = just("unknown").to(Ty::Unknown);
            let nil = just("nil").to(Ty::Nil);
            let boolean = just("boolean").to(Ty::Boolean);
            let string = just("string").to(Ty::String);
            let num = just("number").to(Ty::Number);
            let int = just("integer").to(Ty::Integer);
            let function = just("function").to(Ty::Function);
            let thread = just("thread").to(Ty::Thread);
            let userdata = just("userdata").to(Ty::Userdata);
            let lightuserdata = just("lightuserdata").to(Ty::Lightuserdata);

            fn union_array(
                p: impl Parser<char, Ty, Error = Simple<char>> + Clone,
                inner: impl Parser<char, Ty, Error = Simple<char>>,
            ) -> impl Parser<char, Ty, Error = Simple<char>> {
                choice((
                    // NOTE: Not the way I wanted i.e., Ty::Union(Vec<Ty>) it to be, but it's better than nothing
                    p.clone()
                        .then(just('|').padded().ignore_then(inner))
                        .map(|(x, y)| Ty::Union(Box::new(x), Box::new(y))),
                    p.then(just("[]").repeated())
                        .foldl(|arr, _| Ty::Array(Box::new(arr))),
                ))
            }

            let list_like = ident()
                .padded()
                .then_ignore(colon)
                .then(inner.clone())
                .separated_by(comma)
                .allow_trailing();

            let fun = just("fun")
                .ignore_then(list_like.clone().delimited_by(just('('), just(')')))
                .then(colon.ignore_then(inner.clone().map(Box::new)).or_not())
                .map(|(param, ret)| Ty::Fun(param, ret));

            let table = just("table")
                .ignore_then(
                    just('<')
                        .ignore_then(inner.clone().map(Box::new))
                        .then_ignore(comma)
                        .then(inner.clone().map(Box::new))
                        .then_ignore(just('>'))
                        .or_not(),
                )
                .map(Ty::Table);

            let dict = list_like
                .delimited_by(just('{').then(whitespace()), whitespace().then(just('}')))
                .map(Ty::Dict);

            choice((
                union_array(any, inner.clone()),
                union_array(unknown, inner.clone()),
                union_array(nil, inner.clone()),
                union_array(boolean, inner.clone()),
                union_array(string, inner.clone()),
                union_array(num, inner.clone()),
                union_array(int, inner.clone()),
                union_array(function, inner.clone()),
                union_array(thread, inner.clone()),
                union_array(userdata, inner.clone()),
                union_array(lightuserdata, inner.clone()),
                union_array(fun, inner.clone()),
                union_array(table, inner.clone()),
                union_array(dict, inner),
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
        "{ get: string, set: string }",
        "{ get: fun(a: unknown): unknown, set: fun(a: unknown) }",
        "table<string, string|table<string, string>>",
        "table<string, string>[]",
        "string",
        "any[]",
        "any|any|any",
        "any|string|number",
        "any|string|number|fun(a: string)|table<string, number>|userdata[]",
        "fun(a: string, c: string, d: number): table<string, number[]>[]",
        "fun(a: string, c: string[], d: number[][]): table<string, number>[]",
        "table<string, string|string[]|boolean>[]",
        "fun(a: string, b: string|number|boolean, c: string[], d: number[][]): string|string[]",
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
