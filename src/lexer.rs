// FIXME:
// - [x] takeuntil(end()).to(Tag::Skip)
// - [x] Trailing whitespace in Tag::Comment - It is part of comment
// - array_union! macro is no-go

mod token;
use chumsky::{
    extra,
    prelude::Rich,
    primitive::{any, choice, end, just, one_of},
    recovery::skip_then_retry_until,
    recursive::recursive,
    span::SimpleSpan,
    text::{ident, keyword, newline, whitespace},
    IterParser, Parser,
};
pub use token::*;

macro_rules! array_union {
    ($p: expr, $typ: expr) => {
        $p.foldl(just("[]").repeated(), |arr, _| Ty::Array(Box::new(arr)))
        // NOTE: Not the way I wanted i.e., Ty::Union(Vec<Ty>) it to be, but it's better than nothing
            .foldl(just('|').padded().ignore_then($typ).repeated(), |x, y| {
                Ty::Union(Box::new(x), Box::new(y))
            })
    }
}

const C: [char; 3] = ['.', '_', '-'];

/// Parse emmylua/lua files into rust token
pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<(Token<'src>, SimpleSpan)>, extra::Err<Rich<'src, char, SimpleSpan>>>
{
    let triple = just("---");
    let till_cr = any().and_is(newline().not()).repeated().slice();
    let space = just(' ').repeated().at_least(1);
    let desc = space.ignore_then(till_cr).or_not();
    let name = any()
        .filter(|c: &char| c.is_alphanumeric() || C.contains(c))
        .repeated()
        .slice();

    let block_start = just("[[");
    let block_end = just("]]");

    let optional = just('?').or_not().map(|c| match c {
        Some(_) => Name::Opt as fn(_) -> _,
        None => Name::Req as fn(_) -> _,
    });

    let backtick_string = just('`')
        .ignore_then(any().and_is(just('`').not()).repeated().slice())
        .then_ignore(just('`'));

    let union_literal = choice((
        just('\'')
            .ignore_then(any().and_is(just('\'').not()).repeated().slice())
            .then_ignore(just('\''))
            .map(Member::Literal),
        backtick_string.map(Member::Ident),
    ));

    // Private/Protected/Public
    let public_kw = keyword("public").to(Scope::Public);
    let private_kw = keyword("private")
        .to(Scope::Private)
        .or(keyword("protected").to(Scope::Protected))
        .or(keyword("package").to(Scope::Package));

    let any_typ = keyword("any").to(Ty::Any);
    let unknown = keyword("unknown").to(Ty::Unknown);
    let nil = keyword("nil").to(Ty::Nil);
    let boolean = keyword("boolean").to(Ty::Boolean);
    let string = keyword("string").to(Ty::String);
    let num = keyword("number").to(Ty::Number);
    let int = keyword("integer").to(Ty::Integer);
    let function = keyword("function").to(Ty::Function);
    let thread = keyword("thread").to(Ty::Thread);
    let userdata = keyword("userdata").to(Ty::Userdata);
    let lightuserdata = keyword("lightuserdata").to(Ty::Lightuserdata);

    let ty = recursive(|inner| {
        let comma = just(',').padded();
        let colon = just(':').padded();

        let list_like = ident()
            .padded()
            .then(optional)
            .then(
                colon
                    .ignore_then(inner.clone())
                    .or_not()
                    // NOTE: if param type is missing then LLS treats it as `any`
                    .map(|x| x.unwrap_or(Ty::Any)),
            )
            .map(|((n, attr), t)| (attr(n), t))
            .separated_by(comma)
            .allow_trailing()
            .collect();

        let fun = keyword("fun")
            .ignore_then(
                list_like
                    .clone()
                    .delimited_by(just('(').then(whitespace()), whitespace().then(just(')'))),
            )
            .then(
                colon
                    .ignore_then(inner.clone().separated_by(comma).collect())
                    .or_not(),
            )
            .map(|(param, ret)| Ty::Fun(param, ret));

        let table = keyword("table")
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

        let ty_name = name.map(Ty::Ref);

        let parens = inner
            .clone()
            .delimited_by(just('(').padded(), just(')').padded());

        // Union of string literals: '"g@"'|'"g@$"'
        let string_literal = union_literal.map(Ty::Member);

        choice((
            array_union!(any_typ, inner.clone()),
            array_union!(unknown, inner.clone()),
            array_union!(nil, inner.clone()),
            array_union!(boolean, inner.clone()),
            array_union!(string, inner.clone()),
            array_union!(num, inner.clone()),
            array_union!(int, inner.clone()),
            array_union!(function, inner.clone()),
            array_union!(thread, inner.clone()),
            array_union!(userdata, inner.clone()),
            array_union!(lightuserdata, inner.clone()),
            array_union!(fun, inner.clone()),
            array_union!(table, inner.clone()),
            array_union!(dict, inner.clone()),
            array_union!(parens, inner.clone()),
            array_union!(string_literal, inner.clone()),
            array_union!(ty_name, inner),
        ))
    });

    // ---@brief [[
    // ---@brief ]]
    let brief = keyword("brief").then_ignore(space).ignore_then(
        block_end
            .to(Token::BriefEnd)
            .or(block_start.to(Token::BriefStart)),
    );

    // ---@toc <tag>
    let toc_tag = keyword("toc").then(space).ignore_then(name).map(Token::Toc);

    // ---@mod <name> [desc]
    let mod_tag = keyword("mod")
        .then(space)
        .ignore_then(name)
        .then(desc)
        .map(|(name, desc)| Token::Module(name, desc));

    // ---@divider <char>
    let divider_tag = keyword("divider")
        .then(space)
        .ignore_then(one_of("~-="))
        .map(Token::Divider);

    // ---@param <name[?]|...> <type[|type]> [description]
    let param_tag = keyword("param")
        .then(space)
        .ignore_then(
            ident()
                .then(optional)
                .map(|(name, o)| o(name))
                .or(just("...").map(Name::Req)),
        )
        .then_ignore(space)
        .then(ty.clone())
        .then(desc)
        .map(|((name, typ), desc)| Token::Param(name, typ, desc));

    // ---@return <type> [<name> [comment] | [name] #<comment>]
    let return_tag = keyword("return")
        .ignore_then(space)
        .ignore_then(ty.clone())
        .then(choice((
            newline().to((None, None)),
            space.ignore_then(choice((
                just('#').ignore_then(till_cr).map(|x| (None, Some(x))),
                ident().then(desc).map(|(name, desc)| (Some(name), desc)),
            ))),
        )))
        .map(|(ty, (name, desc))| Token::Return(ty, name, desc));

    // ---@class <name>[: <parent>]
    let class_tag = keyword("class")
        .then(space)
        .ignore_then(name)
        .then(just(':').padded().ignore_then(ident()).or_not())
        .map(|(this, parent)| Token::Class(this, parent));

    // ---@field [public|protected|private] <name[?]> <type> [desc]
    let field_tag = keyword("field")
        .ignore_then(
            space
                .ignore_then(private_kw.clone().or(public_kw.clone()))
                .or_not(),
        )
        .then_ignore(space)
        .then(ident().then(optional))
        .then_ignore(space)
        .then(ty.clone())
        .then(desc)
        .map(|(((scope, (name, opt)), ty), desc)| {
            Token::Field(scope.unwrap_or(Scope::Public), opt(name), ty, desc)
        });

    // -- Simple Alias
    // ---@alias <name> <type>
    // -- Enum alias
    // ---@alias <name>
    let alias_tag = keyword("alias")
        .then(space)
        .ignore_then(name)
        .then(space.ignore_then(ty.clone()).or_not())
        .map(|(name, ty)| Token::Alias(name, ty));

    // ---| '<literal>' [# description]
    // or
    // ---| `<ident>` [# description]
    let enum_member = just('|')
        .then_ignore(space)
        .ignore_then(union_literal)
        .then(
            space
                .ignore_then(just('#').padded().ignore_then(till_cr))
                .or_not(),
        )
        .map(|(t, d)| Token::Variant(t, d));

    // ---@type <type> [desc]
    let type_tag = keyword("type")
        .then(space)
        .ignore_then(ty)
        .then(desc)
        .map(|(ty, desc)| Token::Type(ty, desc));

    // ---@tag <name>
    let tag_tag = keyword("tag").then(space).ignore_then(name).map(Token::Tag);

    // ---@see <name>
    let see_tag = keyword("see")
        .then(space)
        .ignore_then(till_cr.padded())
        .map(Token::See);

    // - Single Line
    // ---@usage [lang] `<TEXT>`
    // - Multi Line
    // ---@usage [lang] [[
    // ---@usage ]]
    let usage_tag = {
        let lang = ident().then_ignore(space).or_not();
        keyword("usage").then(space).ignore_then(choice((
            lang.then(backtick_string)
                .map(|(lang, code)| Token::Usage(lang, code)),
            lang.then_ignore(just("[[")).map(Token::UsageStart),
            just("]]").to(Token::UsageEnd),
        )))
    };

    // ---@export <name>
    let export_tag = keyword("export")
        .then(space)
        .ignore_then(ident())
        .then_ignore(any().repeated())
        .map(Token::Export);

    // ---@private
    let private_tag = private_kw
        .padded()
        .then(choice((
            // eat up all the emmylua, if any, then one valid token
            triple
                .then(till_cr)
                .padded()
                .repeated()
                .ignore_then(ident()),
            // if there is no emmylua, just eat the next token
            // so the next parser won't recognize the code
            ident().padded(),
        )))
        .ignored();

    // emmylua tags
    let tags = just('@').ignore_then(choice((
        brief,
        toc_tag,
        mod_tag,
        divider_tag,
        param_tag,
        return_tag,
        class_tag,
        field_tag,
        alias_tag,
        type_tag,
        tag_tag,
        see_tag,
        usage_tag,
        export_tag,
        private_tag.to(Token::Skip),
        public_kw.to(Token::Skip),
    )));

    // lua-src
    let dotted = just('.')
        .ignore_then(ident())
        .map(Op::Dot)
        .repeated()
        .collect::<Vec<Op<'src>>>();

    // one.two.three =
    let expr = ident()
        .then(dotted)
        .then_ignore(just('=').padded())
        .then(keyword("function").or_not())
        .map(|((name, op), is_fn)| match is_fn {
            Some(_) => Token::Func(name, op),
            None => Token::Expr(name, op),
        });

    // function one.two.three
    // function one.two:three
    // function one:two
    let function = {
        let func_name = dotted
            .then(just(':').ignore_then(ident()).or_not())
            .map(|(mut x, y)| {
                if let Some(c) = y {
                    x.push(Op::Colon(c));
                }
                x
            });
        keyword("function")
            .padded()
            .ignore_then(ident())
            .then(func_name)
            .map(|(name, func_name)| Token::Func(name, func_name))
    };

    // return <ident>\eof
    let final_return = keyword("return")
        .ignore_then(ident().padded())
        .then_ignore(end())
        .map(Token::Export);

    choice((
        triple.ignore_then(choice((tags, enum_member, till_cr.map(Token::Comment)))),
        function,
        expr,
        final_return,
    ))
    .padded()
    // Ignore Useless Nodes
    .recover_with(skip_then_retry_until(any().ignored(), end()))
    .map_with_span(|tok, span| (tok, span))
    .repeated()
    .collect()
    .then_ignore(end())
}
