use chumsky::{
    prelude::{choice, just, take_until},
    text::{self, TextParser},
    Parser,
};

use crate::{common::Desc, impl_parse};

/// ---@brief [[ @comment @brief ]]
#[derive(Debug)]
pub struct Brief(pub String);

impl_parse!(Brief, {
    just("@brief [[")
        .ignore_then(
            take_until(just("@brief ]]").padded().ignored())
                .padded()
                .map(|(x, _)| x.into_iter().collect::<String>()),
        )
        .map(Self)
});

/// ---@tag @comment
#[derive(Debug)]
pub struct Tag(pub String);

impl_parse!(Tag, {
    just("@tag").ignore_then(text::ident().padded()).map(Self)
});

/// ---@class MY_TYPE[:PARENT_TYPE] [@comment]
#[derive(Debug)]
pub struct Class {
    pub name: String,
    // TODO: Visit later
    // parent: Vec<String>,
    pub desc: Desc,
    pub fields: Vec<Field>,
}

impl_parse!(Class, {
    just("@class")
        .ignore_then(text::ident().padded())
        .then(Desc::parse())
        .then(Field::parse().repeated())
        .map(|((name, desc), fields)| Self { name, desc, fields })
});

/// ---@field [public|protected|private] field_name FIELD_TYPE[|OTHER_TYPE] [@comment]
#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub ty: String,
    pub desc: Desc,
}

impl_parse!(Field, {
    just("@field")
        .ignore_then(text::ident().padded())
        .then(text::ident().padded())
        .then(Desc::parse())
        .map(|((name, ty), desc)| Self { name, ty, desc })
});

/// ---@param param_name MY_TYPE[|other_type] [@comment]
#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub ty: String,
    pub desc: Desc,
}

impl_parse!(Param, {
    just("@param")
        .ignore_then(text::ident().padded())
        .then(text::ident().padded())
        .then(Desc::parse())
        .map(|((name, ty), desc)| Self { name, ty, desc })
});

/// ---@type MY_TYPE[|OTHER_TYPE] [@comment]
#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub desc: Desc,
}

impl_parse!(Type, {
    just("@type")
        .ignore_then(text::ident().padded())
        .then(Desc::parse())
        .map(|(name, desc)| Self { name, desc })
});

/// ---@alias NEW_NAME TYPE [@comment]
#[derive(Debug)]
pub struct Alias {
    pub name: String,
    pub ty: String,
    pub desc: Desc,
}

impl_parse!(Alias, {
    just("@alias")
        .ignore_then(text::ident().padded())
        .then(text::ident().padded())
        .then(Desc::parse())
        .map(|((name, ty), desc)| Self { name, ty, desc })
});

/// ---@return MY_TYPE[|OTHER_TYPE] [@comment]
#[derive(Debug)]
pub struct Return {
    pub ty: String,
    pub desc: Desc,
}

impl_parse!(Return, {
    just("@return")
        .ignore_then(text::ident().padded())
        .then(Desc::parse())
        .map(|(ty, desc)| Self { ty, desc })
});

/// ---@see @comment
#[derive(Debug)]
pub struct See(pub String);

impl_parse!(See, {
    just("@see")
        // TODO: replace `Desc` with `Ty`
        .ignore_then(Desc::parse().padded())
        .map(|Desc(x)| Self(x))
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

// #[derive(Debug)]
// pub struct LemmyHelp {
//     pub nodes: Vec<Node>,
// }
//
// impl_parse!(LemmyHelp, {
//     Node::parse().repeated().map(|nodes| Self { nodes })
// });