use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{child_table, impl_parse, section, Name, Object, TagType};

#[derive(Debug, Clone)]
pub struct Func {
    pub name: Name,
    pub scope: String,
    pub desc: Vec<String>,
    pub params: Vec<Object>,
    pub returns: Vec<Object>,
    pub see: Vec<String>,
    pub usage: Option<String>,
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
    .then(select! { TagType::Usage(x) => x }.or_not())
    .then(select! { TagType::Func(n, s) => (n, s) })
    .map(
        |(((((desc, params), returns), see), usage), (name, scope))| Self {
            name,
            scope,
            desc,
            params,
            returns,
            see,
            usage,
        },
    )
});

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut blocks = Vec::with_capacity(3);

        let name = if !self.params.is_empty() {
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

        if let Some(usage) = &self.usage {
            blocks.push(
                child_table!("Usage: ~", [[">"], [&format!("  {}", usage)], ["<"]]).to_string(),
            )
        }

        let desc = self.desc.join("\n");

        let section = section!(&name, self.name.to_string().as_str(), &desc, blocks);

        write!(f, "{}", section)
    }
}
