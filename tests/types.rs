use chumsky::Parser;
use lemmy_help::lexer::{lexer, Member, Name, Ty};

macro_rules! b {
    ($t:expr) => {
        Box::new($t)
    };
}

#[test]
fn types() {
    let type_parse = lexer();

    macro_rules! check {
        ($s:expr, $ty:expr) => {
            assert_eq!(
                type_parse
                    .parse(concat!("---@type ", $s))
                    .into_output()
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap()
                    .0,
                lemmy_help::lexer::Token::Type($ty, None)
            );
        };
    }

    check!("nil", Ty::Nil);
    check!("any", Ty::Any);
    check!("unknown", Ty::Unknown);
    check!("boolean", Ty::Boolean);
    check!("string", Ty::String);
    check!("number", Ty::Number);
    check!("integer", Ty::Integer);
    check!("function", Ty::Function);
    check!("thread", Ty::Thread);
    check!("userdata", Ty::Userdata);
    check!("lightuserdata", Ty::Lightuserdata);
    check!("Any-Thing.El_se", Ty::Ref("Any-Thing.El_se"));

    check!(
        "(string|number|table<string, string[]>)[]",
        Ty::Array(b!(Ty::Union(
            b!(Ty::String),
            b!(Ty::Union(
                b!(Ty::Number),
                b!(Ty::Table(Some((
                    b!(Ty::String),
                    b!(Ty::Array(b!(Ty::String)))
                ))))
            ))
        )))
    );

    check!(
        "table<string, { get: string, set: string }>[]",
        Ty::Array(b!(Ty::Table(Some((
            b!(Ty::String),
            b!(Ty::Dict(vec![
                (Name::Req("get"), Ty::String),
                (Name::Req("set"), Ty::String),
            ]))
        )))))
    );

    check!(
        "table<string, fun(a: string): string>",
        Ty::Table(Some((
            b!(Ty::String),
            b!(Ty::Fun(
                vec![(Name::Req("a"), Ty::String)],
                Some(vec![Ty::String])
            ))
        )))
    );

    check!(
        "table<fun(), table<string, number>>",
        Ty::Table(Some((
            b!(Ty::Fun(vec![], None)),
            b!(Ty::Table(Some((b!(Ty::String), b!(Ty::Number)))))
        )))
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
                (Name::Req("a"), Ty::String),
                (
                    Name::Req("b"),
                    Ty::Union(
                        b!(Ty::String),
                        b!(Ty::Union(b!(Ty::Number), b!(Ty::Boolean)))
                    )
                ),
                (Name::Req("c"), Ty::Array(b!(Ty::Array(b!(Ty::Number))))),
                (Name::Opt("d"), Ty::Ref("SomeClass")),
            ],
            Some(vec![
                Ty::Number,
                Ty::Union(b!(Ty::String), b!(Ty::Array(b!(Ty::String))))
            ])
        )
    );

    // "fun(a: string, b: (string|table<string, number>)[]|boolean, c: string[], d: number[][]): string|string[]",

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
                (Name::Req("a"), Ty::String),
                (Name::Opt("b"), Ty::String),
                (Name::Req("c"), Ty::Function),
                (
                    Name::Req("d"),
                    Ty::Fun(vec![
                        (Name::Req("z"), Ty::String)
                    ], None)
                ),
                (
                    Name::Req("e"),
                    Ty::Union(
                        b!(Ty::String),
                        b!(Ty::Union(
                            b!(Ty::Array(b!(Ty::String))),
                            b!(Ty::Union(
                                b!(Ty::Table(Some((b!(Ty::String), b!(Ty::String))))),
                                b!(Ty::Fun(
                                    vec![(
                                        Name::Req("y"),
                                        Ty::Union(
                                            b!(Ty::Array(b!(Ty::String))),
                                            b!(Ty::Union(
                                                b!(Ty::Dict(vec![
                                                    (Name::Req("get"), Ty::Function)
                                                ])),
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

    check!(
        "{
            inner: string,
            get: fun(a: unknown),
            set: fun(a: unknown),
            __proto__?: { _?: unknown }
        }",
        Ty::Dict(vec![
            (Name::Req("inner"), Ty::String),
            (
                Name::Req("get"),
                Ty::Fun(vec![(Name::Req("a"), Ty::Unknown)], None,)
            ),
            (
                Name::Req("set"),
                Ty::Fun(vec![(Name::Req("a"), Ty::Unknown)], None)
            ),
            (
                Name::Opt("__proto__"),
                Ty::Dict(vec![(Name::Opt("_"), Ty::Unknown)])
            )
        ])
    );

    check!(
        r#"'"g@"'|string[]|'"g@$"'|number"#,
        Ty::Union(
            b!(Ty::Member(Member::Literal(r#""g@""#))),
            b!(Ty::Union(
                b!(Ty::Array(b!(Ty::String))),
                b!(Ty::Union(
                    b!(Ty::Member(Member::Literal(r#""g@$""#))),
                    b!(Ty::Number)
                ))
            ))
        )
    );

    check!(
        "any|any|string|(string|number)[]|fun(a: string)|table<string, number>|userdata[]",
        Ty::Union(
            b!(Ty::Any),
            b!(Ty::Union(
                b!(Ty::Any),
                b!(Ty::Union(
                    b!(Ty::String),
                    b!(Ty::Union(
                        b!(Ty::Array(b!(Ty::Union(b!(Ty::String), b!(Ty::Number))))),
                        b!(Ty::Union(
                            b!(Ty::Fun(vec![(Name::Req("a"), Ty::String)], None)),
                            b!(Ty::Union(
                                b!(Ty::Table(Some((b!(Ty::String), b!(Ty::Number))))),
                                b!(Ty::Array(b!(Ty::Userdata)))
                            ))
                        ))
                    ))
                ))
            ))
        )
    );
}
