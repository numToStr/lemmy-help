use chumsky::Parser;
use lemmy_help::lexer::{Kv, Lexer, Ty};

macro_rules! b {
    ($t:expr) => {
        Box::new($t)
    };
}

#[test]
fn types() {
    let type_parse = Lexer::ty();

    macro_rules! p {
        ($s:expr) => {
            type_parse.parse($s).unwrap()
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

    assert_eq!(
        p!("string[]|string"),
        Ty::Union(b!(Ty::Array(b!(Ty::String))), b!(Ty::String))
    );

    assert_eq!(
        p!(r#"'"g@"'|string[]|'"g@$"'|number"#),
        Ty::Union(
            b!(Ty::Ref(r#""g@""#.into())),
            b!(Ty::Union(
                b!(Ty::Array(b!(Ty::String))),
                b!(Ty::Union(b!(Ty::Ref(r#""g@$""#.into())), b!(Ty::Number)))
            ))
        )
    );

    assert_eq!(
        p!("table<string, string|string[]|boolean>[]"),
        Ty::Array(b!(Ty::Table(Some((
            b!(Ty::String),
            b!(Ty::Union(
                b!(Ty::String),
                b!(Ty::Union(b!(Ty::Array(b!(Ty::String))), b!(Ty::Boolean)))
            ))
        )))))
    );

    assert_eq!(
        p!("fun(
                a: string, b: string|number|boolean, c: number[][], d?: SomeClass
            ): number, string|string[]"),
        Ty::Fun(
            vec![
                Kv::Req("a".into(), Ty::String),
                Kv::Req(
                    "b".into(),
                    Ty::Union(
                        b!(Ty::String),
                        b!(Ty::Union(b!(Ty::Number), b!(Ty::Boolean)))
                    )
                ),
                Kv::Req("c".into(), Ty::Array(b!(Ty::Array(b!(Ty::Number))))),
                Kv::Opt("d".into(), Ty::Ref("SomeClass".into())),
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

    let src = "fun(
        a: string,
        b?: string,
        c: function,
        d: fun(z: string),
        e: string|string[]|table<string, string>|fun(y: string[]|{ get: function }|string): string
    ): table<string, string>";

    assert_eq!(
        p!(src),
        Ty::Fun(
            vec![
                Kv::Req("a".into(), Ty::String),
                Kv::Opt("b".into(), Ty::String),
                Kv::Req("c".into(), Ty::Function),
                Kv::Req(
                    "d".into(),
                    Ty::Fun(vec![Kv::Req("z".into(), Ty::String)], None)
                ),
                Kv::Req(
                    "e".into(),
                    Ty::Union(
                        b!(Ty::String),
                        b!(Ty::Union(
                            b!(Ty::Array(b!(Ty::String))),
                            b!(Ty::Union(
                                b!(Ty::Table(Some((b!(Ty::String), b!(Ty::String))))),
                                b!(Ty::Fun(
                                    vec![Kv::Req(
                                        "y".into(),
                                        Ty::Union(
                                            b!(Ty::Array(b!(Ty::String))),
                                            b!(Ty::Union(
                                                b!(Ty::Dict(vec![Kv::Req(
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
