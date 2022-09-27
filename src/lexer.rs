use std::ops::Range;

use chumsky::{
    prelude::{any, choice, end, filter, just, take_until, Simple},
    text::{ident, keyword, newline, TextParser},
    Parser,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Kind {
    Dot,
    Colon,
    Local,
}

impl Kind {
    pub fn as_char(&self) -> char {
        match self {
            Self::Dot => '.',
            Self::Colon => ':',
            Self::Local => '#',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scope {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagType {
    /// ```lua
    /// ---@toc <name>
    /// ```
    Toc(String),
    /// ```lua
    /// ---@mod <name> [desc]
    /// ```
    Module(String, Option<String>),
    /// ```lua
    /// ---@divider <char>
    /// ```
    Divider(char),
    /// ```lua
    /// function one.two() end
    /// one.two = function() end
    /// ```
    Func {
        prefix: Option<String>,
        name: String,
        kind: Kind,
    },
    /// ```lua
    /// one = 1
    /// one.two = 12
    /// ```
    Expr {
        prefix: Option<String>,
        name: String,
        kind: Kind,
    },
    /// ```lua
    /// ---@export <module>
    /// or
    /// return <module>\eof
    /// ```
    Export(String),
    /// ```lua
    /// ---@brief [[
    /// ```
    BriefStart,
    /// ```lua
    /// ---@brief ]]
    /// ```
    BriefEnd,
    /// ```lua
    /// ---@param <name[?]> <type[|type...]> [description]
    /// ```
    Param {
        name: String,
        ty: String,
        desc: Option<String>,
    },
    /// ```lua
    /// ---@return <type> [<name> [comment] | [name] #<comment>]
    /// ```
    Return {
        ty: String,
        name: Option<String>,
        desc: Option<String>,
    },
    /// ```lua
    /// ---@class <name>
    /// ```
    Class(String),
    /// ```lua
    /// ---@field [public|private|protected] <name> <type> [description]
    /// ```
    Field {
        scope: Scope,
        name: String,
        ty: String,
        desc: Option<String>,
    },
    /// ```lua
    /// -- Simple Alias
    /// ---@alias <name> <type>
    ///
    /// -- Enum alias
    /// ---@alias <name>
    /// ```
    Alias(String, Option<String>),
    /// ```lua
    /// ---| '<value>' [# description]
    /// ```
    Variant(String, Option<String>),
    /// ```lua
    /// ---@type <type> [desc]
    /// ```
    Type(String, Option<String>),
    /// ```lua
    /// ---@tag <name>
    /// ```
    Tag(String),
    /// ```lua
    /// ---@see <name>
    /// ```
    See(String),
    /// ```lua
    /// ---@usage `<code>`
    /// ```
    Usage(String),
    /// ```lua
    /// ---@usage [[
    /// ```
    UsageStart,
    /// ```lua
    /// ---@usage ]]
    /// ```
    UsageEnd,
    /// ```lua
    /// ---TEXT
    /// ```
    Comment(String),
    /// Text nodes which are not needed
    Skip,
}

type Spanned = (TagType, Range<usize>);

#[derive(Debug)]
pub struct Lexer;

impl Lexer {
    /// Parse emmylua/lua files into rust token
    pub fn parse(src: &str) -> Result<Vec<Spanned>, Vec<Simple<char>>> {
        let triple = just("---");
        let space = just(' ').repeated().at_least(1);
        let till_eol = take_until(newline());

        let comment = till_eol.map(|(x, _)| x.iter().collect());
        let desc = space.ignore_then(comment).or_not();

        // Source: https://github.com/sumneko/lua-language-server/wiki/Annotations#documenting-types
        // A TYPE could be
        // - primary = string|number|boolean
        // - fn = func(...):string
        // - enum = "one"|"two"|"three"
        // - or: primary (| primary)+
        // - optional = primary?
        // - table = table<string, string>
        // - array = primary[]
        let ty = filter(|x: &char| !x.is_whitespace()).repeated().collect();

        let scope = choice((
            keyword("public").to(Scope::Public),
            keyword("protected").to(Scope::Protected),
            keyword("private").to(Scope::Private),
        ));

        let private = just("private")
            .then_ignore(newline())
            .then_ignore(choice((
                // eat up all the emmylua, if any, then one valid token
                triple
                    .then(till_eol)
                    .padded()
                    .repeated()
                    .ignore_then(ident()),
                // if there is no emmylua, just eat the next token
                // so the next parser won't recognize the code
                ident().padded(),
            )))
            .ignored();

        let variant = just('|')
            .then_ignore(space)
            .ignore_then(
                just('\'')
                    .ignore_then(filter(|c| c != &'\'').repeated())
                    .then_ignore(just('\''))
                    .collect(),
            )
            .then(
                space
                    .ignore_then(just('#').ignore_then(space).ignore_then(comment))
                    .or_not(),
            )
            .map(|(t, d)| TagType::Variant(t, d));

        let tag = just('@').ignore_then(choice((
            private.to(TagType::Skip),
            just("toc")
                .ignore_then(space)
                .ignore_then(comment)
                .map(TagType::Toc),
            just("mod")
                .then_ignore(space)
                .ignore_then(ty)
                .then(desc)
                .map(|(name, desc)| TagType::Module(name, desc)),
            just("divider")
                .ignore_then(space)
                .ignore_then(any())
                .map(TagType::Divider),
            just("brief").ignore_then(space).ignore_then(choice((
                just("[[").to(TagType::BriefStart),
                just("]]").to(TagType::BriefEnd),
            ))),
            just("param")
                .ignore_then(space)
                .ignore_then(ty) // I am using `ty` here because param can have `?`
                .then_ignore(space)
                .then(ty)
                .then(desc)
                .map(|((name, ty), desc)| TagType::Param { name, ty, desc }),
            just("return")
                .ignore_then(space)
                .ignore_then(ty)
                .then(choice((
                    newline().to((None, None)),
                    space.ignore_then(choice((
                        just('#').ignore_then(comment).map(|x| (None, Some(x))),
                        ident().then(desc).map(|(name, desc)| (Some(name), desc)),
                    ))),
                )))
                .map(|(ty, (name, desc))| TagType::Return { ty, name, desc }),
            just("class")
                .ignore_then(space)
                .ignore_then(ident())
                .map(TagType::Class),
            just("field")
                .ignore_then(space.ignore_then(scope).or_not())
                .then_ignore(space)
                .then(ident())
                .then_ignore(space)
                .then(ty)
                .then(desc)
                .map(|(((scope, name), ty), desc)| TagType::Field {
                    scope: scope.unwrap_or(Scope::Public),
                    name,
                    ty,
                    desc,
                }),
            just("alias")
                .ignore_then(space)
                .ignore_then(ident())
                .then(space.ignore_then(ty).or_not())
                .map(|(name, ty)| TagType::Alias(name, ty)),
            just("type")
                .ignore_then(space)
                .ignore_then(ty)
                .then(desc)
                .map(|(ty, desc)| TagType::Type(ty, desc)),
            just("tag")
                .ignore_then(space)
                .ignore_then(comment)
                .map(TagType::Tag),
            just("see")
                .ignore_then(space)
                .ignore_then(comment)
                .map(TagType::See),
            just("usage").ignore_then(space).ignore_then(choice((
                just("[[").to(TagType::UsageStart),
                just("]]").to(TagType::UsageEnd),
                just('`')
                    .ignore_then(filter(|c| *c != '`').repeated())
                    .then_ignore(just('`'))
                    .collect()
                    .map(TagType::Usage),
            ))),
            just("export")
                .ignore_then(space)
                .ignore_then(ident())
                .then_ignore(take_until(end()))
                .map(TagType::Export),
        )));

        let local = keyword("local").padded();
        let func = keyword("function").padded();
        let assign = just('=').padded();

        let dotted = choice((
            ident()
                .then(choice((just('.').to(Kind::Dot), just(':').to(Kind::Colon))))
                .then(ident())
                .map(|((prefix, scope), name)| (Some(prefix), scope, name)),
            ident().map(|name| (None, Kind::Local, name)),
        ));

        let expr = dotted.clone().then_ignore(assign);

        choice((
            triple.ignore_then(choice((
                tag,
                variant,
                newline().to(TagType::Comment(String::new())),
                comment.map(TagType::Comment),
            ))),
            local.ignore_then(choice((
                func.clone().ignore_then(ident()).map(|name| TagType::Func {
                    name,
                    prefix: None,
                    kind: Kind::Local,
                }),
                ident().then_ignore(assign).map(|name| TagType::Expr {
                    name,
                    prefix: None,
                    kind: Kind::Local,
                }),
            ))),
            func.clone()
                .ignore_then(dotted)
                .map(|(prefix, kind, name)| TagType::Func { prefix, name, kind }),
            choice((
                expr.clone()
                    .then_ignore(func)
                    .map(|(prefix, kind, name)| TagType::Func { prefix, name, kind }),
                expr.map(|(prefix, kind, name)| TagType::Expr { prefix, name, kind }),
            )),
            keyword("return")
                .ignore_then(ident().padded())
                .then_ignore(end())
                .map(TagType::Export),
            till_eol.to(TagType::Skip),
        ))
        .padded()
        .map_with_span(|t, r| (t, r))
        .repeated()
        .parse(src)
    }
}
