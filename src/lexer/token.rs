use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Member<'m> {
    Literal(&'m str),
    Ident(&'m str),
}

impl Display for Member<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(lit) => f.write_str(&format!(
                r#""{}""#,
                lit.trim_start_matches('"').trim_end_matches('"')
            )),
            Self::Ident(ident) => f.write_str(ident),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token<'tt> {
    /// ```lua
    /// ---@toc <name>
    /// ```
    Toc(&'tt str),
    /// ```lua
    /// ---@mod <name> [desc]
    /// ```
    Module(&'tt str, Option<&'tt str>),
    /// ```lua
    /// ---@divider <char>
    /// ```
    Divider(char),
    /// ```lua
    /// function one.two() end
    /// one.two = function() end
    /// ```
    Func(&'tt str, Vec<Op<'tt>>),
    /// ```lua
    /// one = 1
    /// one.two = 12
    /// ```
    Expr(&'tt str, Vec<Op<'tt>>),
    /// ```lua
    /// ---@export <module>
    /// or
    /// return <module>\eof
    /// ```
    Export(&'tt str),
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
    Param(Name<'tt>, Ty<'tt>, Option<&'tt str>),
    /// ```lua
    /// ---@return <type> [<name> [comment] | [name] #<comment>]
    /// ```
    Return(Ty<'tt>, Option<&'tt str>, Option<&'tt str>),
    /// ```lua
    /// ---@class <name>[: <parent>]
    /// ```
    Class(&'tt str, Option<&'tt str>),
    /// ```lua
    /// ---@field [public|private|protected] <name[?]> <type> [description]
    /// ```
    Field(Scope, Name<'tt>, Ty<'tt>, Option<&'tt str>),
    /// ```lua
    /// -- Simple Alias
    /// ---@alias <name> <type>
    ///
    /// -- Enum alias
    /// ---@alias <name>
    /// ```
    Alias(&'tt str, Option<Ty<'tt>>),
    /// ```lua
    /// ---| '<literal>' [# description]
    ///
    /// -- or
    ///
    /// ---| `<ident>` [# description]
    /// ```
    Variant(Member<'tt>, Option<&'tt str>),
    /// ```lua
    /// ---@type <type> [desc]
    /// ```
    Type(Ty<'tt>, Option<&'tt str>),
    /// ```lua
    /// ---@tag <name>
    /// ```
    Tag(&'tt str),
    /// ```lua
    /// ---@see <name>
    /// ```
    See(&'tt str),
    /// ```lua
    /// ---@usage [lang] `<code>`
    /// ```
    Usage(Option<&'tt str>, &'tt str),
    /// ```lua
    /// ---@usage [lang] [[
    /// ```
    UsageStart(Option<&'tt str>),
    /// ```lua
    /// ---@usage ]]
    /// ```
    UsageEnd,
    /// ```lua
    /// ---TEXT
    /// ```
    Comment(&'tt str),
    /// Text nodes which are not needed
    Skip,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Op<'op> {
    Dot(&'op str),
    Colon(&'op str),
}

impl Display for Op<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dot(dot) => {
                f.write_str(".")?;
                f.write_str(dot)
            }
            Self::Colon(colon) => {
                f.write_str(":")?;
                f.write_str(colon)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scope {
    Public,
    Private,
    Protected,
    Package,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Name<'nm> {
    Req(&'nm str),
    Opt(&'nm str),
}

impl Display for Name<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Req(n) => f.write_str(n),
            Self::Opt(n) => {
                f.write_str(n)?;
                f.write_str("?")
            }
        }
    }
}

// Source: https://github.com/sumneko/lua-language-server/wiki/Annotations#documenting-types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty<'ty> {
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
    Ref(&'ty str),
    Member(Member<'ty>),
    Array(Box<Ty<'ty>>),
    Table(Option<(Box<Ty<'ty>>, Box<Ty<'ty>>)>),
    Fun(Vec<(Name<'ty>, Ty<'ty>)>, Option<Vec<Ty<'ty>>>),
    Dict(Vec<(Name<'ty>, Ty<'ty>)>),
    Union(Box<Ty<'ty>>, Box<Ty<'ty>>),
}

impl Display for Ty<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn list_like(args: &[(Name, Ty)]) -> String {
            args.iter()
                .map(|(n, t)| format!("{n}:{t}"))
                .collect::<Vec<String>>()
                .join(",")
        }

        match self {
            Self::Nil => f.write_str("nil"),
            Self::Any => f.write_str("any"),
            Self::Unknown => f.write_str("unknown"),
            Self::Boolean => f.write_str("boolean"),
            Self::String => f.write_str("string"),
            Self::Number => f.write_str("number"),
            Self::Integer => f.write_str("integer"),
            Self::Function => f.write_str("function"),
            Self::Thread => f.write_str("thread"),
            Self::Userdata => f.write_str("userdata"),
            Self::Lightuserdata => f.write_str("lightuserdata"),
            Self::Ref(id) => f.write_str(id),
            Self::Array(ty) => {
                f.write_str(&ty.to_string())?;
                f.write_str("[]")
            }
            Self::Table(kv) => match kv {
                Some((k, v)) => {
                    f.write_str("table<")?;
                    f.write_str(&k.to_string())?;
                    f.write_str(",")?;
                    f.write_str(&v.to_string())?;
                    f.write_str(">")
                }
                None => f.write_str("table"),
            },
            Self::Fun(args, ret) => {
                f.write_str("fun(")?;
                f.write_str(&list_like(args))?;
                f.write_str(")")?;
                if let Some(ret) = ret {
                    f.write_str(":")?;
                    f.write_str(
                        &ret.iter()
                            .map(|r| r.to_string())
                            .collect::<Vec<String>>()
                            .join(","),
                    )?;
                }
                Ok(())
            }
            Self::Dict(kv) => {
                f.write_str("{")?;
                f.write_str(&list_like(kv))?;
                f.write_str("}")
            }
            Self::Union(rhs, lhs) => {
                f.write_str(&rhs.to_string())?;
                f.write_str("|")?;
                f.write_str(&lhs.to_string())
            }
            Self::Member(mem) => mem.fmt(f),
        }
    }
}
