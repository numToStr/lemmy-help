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
        let mut blocks = Vec::with_capacity(2);

        let tag = if !self.params.is_empty() {
            blocks.push(
                child_table!(
                    "Parameters: ~",
                    self.params.iter().map(|field| {
                        [
                            format!("{{{}}}", field.name),
                            format!("({})", field.ty),
                            field.desc.clone().unwrap_or_default(),
                        ]
                    })
                )
                .to_string(),
            );

            let args = self
                .params
                .iter()
                .map(|x| format!("{{{}}}", x.name))
                .collect::<Vec<String>>()
                .join(", ");

            format!("{}({})", self.name, args)
        } else {
            format!("{}()", self.name)
        };

        if !self.returns.is_empty() {
            blocks.push(
                child_table!(
                    "Returns: ~",
                    self.returns.iter().map(|r| [
                        format!("{{{}}}", r.ty),
                        r.desc.clone().unwrap_or_else(|| r.name.clone())
                    ])
                )
                .to_string(),
            )
        };

        let section = section!(
            &tag,
            self.name.as_str(),
            self.desc
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
                .as_str(),
            blocks
        );

        write!(f, "{}", section)
    }
}
