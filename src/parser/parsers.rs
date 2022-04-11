use chumsky::{
    prelude::{choice, just, take_until, Simple},
    text::TextParser,
    Parser,
};

use crate::{
    impl_parse,
    parser::common::{Desc, Name, Ty},
    Class, Return,
};

/// ---@brief [[ TEXT @brief ]]
#[derive(Debug)]
pub struct Brief(pub String);

impl_parse!(Brief, {
    just("---@brief [[").ignore_then(
        take_until(just("---@brief ]]").padded().ignored())
            .padded()
            .map(|(x, _)| Self(x.into_iter().collect())),
    )
});

/// ---@tag @comment
#[derive(Debug)]
pub struct Tag {
    pub name: Name,
}

impl_parse!(
    Tag,
    just("---@tag")
        .ignore_then(Name::parse())
        .map(|name| Self { name })
);

/// ---@field [public|protected|private] field_name FIELD_TYPE[|OTHER_TYPE] [@comment]
#[derive(Debug)]
pub struct Field {
    pub name: Name,
    pub ty: Ty,
    pub desc: Option<Desc>,
}

impl_parse!(Field, {
    just("---@field")
        .ignore_then(Name::parse())
        .then(Ty::parse())
        .then(Desc::parse())
        .map(|((name, ty), desc)| Self { name, ty, desc })
});

/// ---@param param_name MY_TYPE[|other_type] [@comment]
#[derive(Debug)]
pub struct Param {
    pub name: Name,
    pub ty: Ty,
    pub desc: Option<Desc>,
}

impl_parse!(Param, {
    just("---@param")
        .ignore_then(Name::parse())
        .then(Ty::parse())
        .then(Desc::parse())
        .map(|((name, ty), desc)| Self { name, ty, desc })
});

/// ---@type MY_TYPE[|OTHER_TYPE] [@comment]
#[derive(Debug)]
pub struct Type {
    pub name: Name,
    pub desc: Option<Desc>,
}

impl_parse!(Type, {
    just("---@type")
        .ignore_then(Name::parse())
        .then(Desc::parse())
        .map(|(name, desc)| Self { name, desc })
});

/// ---@alias NEW_NAME TYPE [@comment]
#[derive(Debug)]
pub struct Alias {
    pub name: Name,
    pub ty: Ty,
    pub desc: Option<Desc>,
}

impl_parse!(Alias, {
    just("---@alias")
        .ignore_then(Name::parse())
        .then(Ty::parse())
        .then(Desc::parse())
        .map(|((name, ty), desc)| Self { name, ty, desc })
});

/// ---@see @comment
#[derive(Debug)]
pub struct See(pub String);

impl_parse!(See, {
    just("---@see")
        .ignore_then(Ty::parse())
        .map(|Ty(x)| Self(x))
});

#[derive(Debug)]
pub enum Node {
    Brief(Brief),
    Tag(Tag),
    Class(Class),
    Type(Type),
    Param(Param),
    Return(Return),
    Alias(Alias),
    See(See),
}

impl_parse!(Node, {
    choice((
        Brief::parse().map(Self::Brief),
        Tag::parse().map(Self::Tag),
        Class::parse().map(Self::Class),
        Return::parse().map(Self::Return),
        Type::parse().map(Self::Type),
        Param::parse().map(Self::Param),
        Alias::parse().map(Self::Alias),
        See::parse().map(Self::See),
    ))
});

#[derive(Debug)]
pub struct LemmyHelp {
    pub nodes: Vec<Node>,
}

impl LemmyHelp {
    pub fn parse(src: &str) -> Result<Self, Vec<Simple<char>>> {
        Node::parse()
            .repeated()
            .parse(src)
            .map(|nodes| Self { nodes })
    }
}
