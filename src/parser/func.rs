use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{child_table, impl_parse, section, Comment, Object, TagType};

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub desc: Vec<Comment>,
    pub params: Vec<Object>,
    pub returns: Vec<Object>,
}

impl_parse!(Func, {
    Comment::parse()
        .repeated()
        .then(select! { TagType::Param(x) => x }.repeated())
        .then(select! { TagType::Return(x) => x }.repeated())
        .then(select! { TagType::Name(x) => x })
        .map(|(((desc, params), returns), name)| Self {
            name,
            desc,
            params,
            returns,
        })
});

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args: String = self
            .params
            .iter()
            .map(|x| format!("{{{}}}", x.name))
            .collect::<Vec<String>>()
            .join(", ");
        let name = format!("{}({})", self.name, args);

        let params = child_table!(
            "Parameters: ~",
            self.params.iter().map(|field| {
                [
                    format!("{{{}}}", field.name),
                    format!("({})", field.ty),
                    field.desc.clone().unwrap_or_default(),
                ]
            })
        );

        let returns = child_table!(
            "Returns: ~",
            self.returns.iter().map(|r| [
                format!("{{{}}}", r.ty),
                r.desc.clone().unwrap_or_else(|| r.name.clone())
            ])
        );

        let section = section!(
            &name,
            self.name.as_str(),
            self.desc
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
                .as_str(),
            params.to_string().as_str(),
            returns.to_string().as_str()
        );

        write!(f, "{}", section)
    }
}
