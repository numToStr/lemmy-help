use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{parser, Kind, Prefix, See, Table, TagType, Usage};

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: String,
    pub desc: Vec<String>,
}

parser!(Param, {
    select! { TagType::Param { name, ty, desc } => (name, ty, desc) }
        .then(
            select! {
                TagType::Comment(x) => x,
                TagType::Empty => String::new()
            }
            .repeated(),
        )
        .map(|((name, ty, desc), extra)| {
            let desc = match desc {
                Some(d) => Vec::from([d])
                    .into_iter()
                    .chain(extra.into_iter())
                    .collect(),
                None => extra,
            };

            Self { name, ty, desc }
        })
});

#[derive(Debug, Clone)]
pub struct Return {
    pub ty: String,
    pub name: Option<String>,
    pub desc: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub kind: Kind,
    pub prefix: Prefix,
    pub desc: Vec<String>,
    pub params: Vec<Param>,
    pub returns: Vec<Return>,
    pub see: See,
    pub usage: Option<Usage>,
}

parser!(Func, {
    select! {
        TagType::Comment(x) => x,
        TagType::Empty => String::new()
    }
    .repeated()
    .then(Param::parse().repeated())
    .then(select! { TagType::Return { ty, name, desc } => Return { ty, name, desc } }.repeated())
    .then(See::parse())
    .then(Usage::parse().or_not())
    .then(select! { TagType::Func { prefix, name, kind } => (prefix, name, kind) })
    .map(
        |(((((desc, params), returns), see), usage), (prefix, name, kind))| Self {
            name,
            kind,
            prefix: Prefix {
                left: prefix.clone(),
                right: prefix,
            },
            desc,
            params,
            returns,
            see,
            usage,
        },
    )
});

impl Func {
    pub fn rename_tag(&mut self, tag: String) {
        self.prefix.right = Some(tag);
    }

    pub fn is_public(&self, export: &str) -> bool {
        self.kind != Kind::Local && self.prefix.left.as_deref() == Some(export)
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::{description, header};

        let name = if !self.params.is_empty() {
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

        header!(
            f,
            format!(
                "{}{}{name}",
                self.prefix.left.as_deref().unwrap_or_default(),
                self.kind.as_char()
            ),
            format!(
                "{}{}{}",
                self.prefix.right.as_deref().unwrap_or_default(),
                self.kind.as_char(),
                self.name
            )
        )?;

        description!(f, &self.desc.join("\n"))?;
        writeln!(f)?;

        if !self.params.is_empty() {
            description!(f, "Parameters: ~")?;

            let mut table = Table::new();

            for param in &self.params {
                table.add_row([
                    &format!("{{{}}}", param.name),
                    &format!("({})", param.ty),
                    &param.desc.join("\n"),
                ]);
            }

            writeln!(f, "{table}")?;
        }

        if !self.returns.is_empty() {
            description!(f, "Returns: ~")?;

            let mut table = Table::new();

            for entry in &self.returns {
                table.add_row([
                    &format!("{{{}}}", entry.ty),
                    entry
                        .desc
                        .as_deref()
                        .unwrap_or_else(|| entry.name.as_deref().unwrap_or_default()),
                ]);
            }

            writeln!(f, "{table}")?;
        }

        if !self.see.refs.is_empty() {
            writeln!(f, "{}", self.see)?;
        }

        if let Some(usage) = &self.usage {
            writeln!(f, "{usage}")?;
        }

        Ok(())
    }
}
