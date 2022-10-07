use std::fmt::Display;

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
        optional: bool,
        ty: Ty,
        desc: Option<String>,
    },
    /// ```lua
    /// ---@return <type> [<name> [comment] | [name] #<comment>]
    /// ```
    Return {
        ty: Ty,
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
        ty: Ty,
        desc: Option<String>,
    },
    /// ```lua
    /// -- Simple Alias
    /// ---@alias <name> <type>
    ///
    /// -- Enum alias
    /// ---@alias <name>
    /// ```
    Alias(String, Option<Ty>),
    /// ```lua
    /// ---| '<value>' [# description]
    /// ```
    Variant(String, Option<String>),
    /// ```lua
    /// ---@type <type> [desc]
    /// ```
    Type(Ty, Option<String>),
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

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ty-ty")
    }
}
