use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{impl_parse, Prefix, Scope, See, TagType, Usage};

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: String,
    pub desc: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub ty: String,
    pub name: Option<String>,
    pub desc: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub scope: Scope,
    pub prefix: Prefix,
    pub desc: Vec<String>,
    pub params: Vec<Param>,
    pub returns: Vec<Return>,
    pub see: See,
    pub usage: Option<Usage>,
}

impl_parse!(Func, {
    select! {
        TagType::Comment(x) => x,
        TagType::Empty => "\n".to_string()
    }
    .repeated()
    .then(select! { TagType::Param { name, ty, desc } => Param { name, ty, desc } }.repeated())
    .then(select! { TagType::Return { ty, name, desc } => Return { ty, name, desc } }.repeated())
    .then(See::parse())
    .then(Usage::parse().or_not())
    .then(select! { TagType::Func { prefix, name, scope } => (prefix, name, scope) })
    .map(
        |(((((desc, params), returns), see), usage), (prefix, name, scope))| Self {
            name,
            scope,
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
        self.scope != Scope::Local && self.prefix.left.as_deref() == Some(export)
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
                self.scope
            ),
            format!(
                "{}{}{}",
                self.prefix.right.as_deref().unwrap_or_default(),
                self.scope,
                self.name
            )
        )?;

        description!(f, &self.desc.join("\n"))?;
        writeln!(f)?;

        if !self.params.is_empty() {
            description!(f, "Parameters: ~")?;

            let mut table = tabular::Table::new("        {:<}  {:<}  {:<}");

            for param in &self.params {
                table.add_row(
                    tabular::Row::new()
                        .with_cell(&format!("{{{}}}", param.name))
                        .with_cell(&format!("({})", param.ty))
                        .with_cell(param.desc.as_deref().unwrap_or_default()),
                );
            }

            writeln!(f, "{}", table)?;
        }

        if !self.returns.is_empty() {
            description!(f, "Returns: ~")?;

            let mut table = tabular::Table::new("        {:<}  {:<}");

            for entry in &self.returns {
                table.add_row(
                    tabular::Row::new()
                        .with_cell(&format!("{{{}}}", entry.ty))
                        .with_cell(
                            entry
                                .desc
                                .as_deref()
                                .unwrap_or_else(|| entry.name.as_deref().unwrap_or_default()),
                        ),
                );
            }

            writeln!(f, "{}", table)?;
        }

        if !self.see.refs.is_empty() {
            writeln!(f, "{}", self.see)?;
        }

        if let Some(usage) = &self.usage {
            writeln!(f, "{usage}")?;
        }

        write!(f, "")
    }
}
