use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{child_table, impl_parse, section, Object, TagType};

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub scope: String,
    pub desc: Vec<String>,
    pub params: Vec<Object>,
    pub returns: Vec<Object>,
    pub see: Vec<String>,
}

impl_parse!(Func, {
    select! {
        TagType::Comment(x) => x,
        TagType::Empty => "\n".to_string()
    }
    .repeated()
    .then(select! { TagType::Param(x) => x }.repeated())
    .then(select! { TagType::Return(x) => x }.repeated())
    .then(select! { TagType::See(x) => x }.repeated())
    .then(select! { TagType::Func(n, s) => (n, s) })
    .map(|((((desc, params), returns), see), (name, scope))| Self {
        name,
        scope,
        desc,
        params,
        returns,
        see,
    })
});

impl Func {
    pub fn is_public(&self) -> bool {
        &self.scope == "public"
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut blocks = Vec::with_capacity(3);

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

        if !self.see.is_empty() {
            blocks.push(
                child_table!("See: ~", self.see.iter().map(|s| [format!("|{}|", s)])).to_string(),
            )
        }

        let section = section!(
            &tag,
            self.name.as_str(),
            self.desc.join(" ").as_str(),
            blocks
        );

        write!(f, "{}", section)
    }
}
