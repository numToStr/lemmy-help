use chumsky::Parser;
use lemmy_help::lexer::{Lexer, Ty, TypeVal};

macro_rules! b {
    ($t:expr) => {
        Box::new($t)
    };
}

#[test]
fn types() {
    let type_parse = Lexer::init();

    macro_rules! check {
        ($s:expr, $ty:expr) => {
            assert_eq!(
                type_parse
                    .parse(concat!("---@type ", $s))
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap()
                    .0,
                lemmy_help::lexer::TagType::Type($ty, None)
            );
        };
    }

    //         "table<string, fun(a: string): string>",
    //         "table<fun(), table<string, number>>",
    //         "table<string, fun(a: string, b: table<string, boolean>)>",
    //         "{ get: string, set: string }",
    //         "{ get: fun(a: unknown): unknown, set: fun(a: unknown) }",
    //         "table<string, string|table<string, string>>",
    //         "table<string, string>[]",
    //         "string",
    //         "any[]",
    //         "any|any|any",
    //         "any|string|number",
    //         "any|string|number|fun(a: string)|table<string, number>|userdata[]",
    //         "fun(a: string, c: string, d: number): table<string, number[]>[]",
    //         "fun(a: string, c: string[], d: number[][]): table<string, number>[]",

    check!(
        "string[]|string",
        Ty::Union(b!(Ty::Array(b!(Ty::String))), b!(Ty::String))
    );

    check!(
        r#"'"g@"'|string[]|'"g@$"'|number"#,
        Ty::Union(
            b!(Ty::Ref(r#""g@""#.into())),
            b!(Ty::Union(
                b!(Ty::Array(b!(Ty::String))),
                b!(Ty::Union(b!(Ty::Ref(r#""g@$""#.into())), b!(Ty::Number)))
            ))
        )
    );

    check!(
        "table<string, string|string[]|boolean>[]",
        Ty::Array(b!(Ty::Table(Some((
            b!(Ty::String),
            b!(Ty::Union(
                b!(Ty::String),
                b!(Ty::Union(b!(Ty::Array(b!(Ty::String))), b!(Ty::Boolean)))
            ))
        )))))
    );

    check!(
        "fun(
            a: string, b: string|number|boolean, c: number[][], d?: SomeClass
        ): number, string|string[]",
        Ty::Fun(
            vec![
                TypeVal::Req("a".into(), Ty::String),
                TypeVal::Req(
                    "b".into(),
                    Ty::Union(
                        b!(Ty::String),
                        b!(Ty::Union(b!(Ty::Number), b!(Ty::Boolean)))
                    )
                ),
                TypeVal::Req("c".into(), Ty::Array(b!(Ty::Array(b!(Ty::Number))))),
                TypeVal::Opt("d".into(), Ty::Ref("SomeClass".into())),
            ],
            Some(vec![
                Ty::Number,
                Ty::Union(b!(Ty::String), b!(Ty::Array(b!(Ty::String))))
            ])
        )
    );

    //         "table<string, { get: string, set: string }>[]",
    //         "(string|number|table<string, string[]>)[]",
    //         "table<string, string|string[]|boolean>[]",
    //         "fun(a: string, b: (string|table<string, number>)[]|boolean, c: string[], d: number[][]): string|string[]",

    check!(
        "fun(
            a: string,
            b?: string,
            c: function,
            d: fun(z: string),
            e: string|string[]|table<string, string>|fun(y: string[]|{ get: function }|string): string
        ): table<string, string>",
        Ty::Fun(
            vec![
                TypeVal::Req("a".into(), Ty::String),
                TypeVal::Opt("b".into(), Ty::String),
                TypeVal::Req("c".into(), Ty::Function),
                TypeVal::Req(
                    "d".into(),
                    Ty::Fun(vec![TypeVal::Req("z".into(), Ty::String)], None)
                ),
                TypeVal::Req(
                    "e".into(),
                    Ty::Union(
                        b!(Ty::String),
                        b!(Ty::Union(
                            b!(Ty::Array(b!(Ty::String))),
                            b!(Ty::Union(
                                b!(Ty::Table(Some((b!(Ty::String), b!(Ty::String))))),
                                b!(Ty::Fun(
                                    vec![TypeVal::Req(
                                        "y".into(),
                                        Ty::Union(
                                            b!(Ty::Array(b!(Ty::String))),
                                            b!(Ty::Union(
                                                b!(Ty::Dict(vec![TypeVal::Req(
                                                    "get".into(),
                                                    Ty::Function
                                                )])),
                                                b!(Ty::String)
                                            ))
                                        )
                                    ),],
                                    Some(vec![Ty::String])
                                ))
                            ))
                        ))
                    )
                )
            ],
            Some(vec![Ty::Table(Some((b!(Ty::String), b!(Ty::String))))])
        )
    );
}
