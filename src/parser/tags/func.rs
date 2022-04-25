use std::fmt::Display;

use chumsky::{select, Parser};

use crate::{impl_parse, see, usage, Prefix, Scope, TagType};

#[derive(Debug, Clone)]
pub struct Param {
    name: String,
    ty: String,
    desc: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Return {
    ty: String,
    name: Option<String>,
    desc: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub scope: Scope,
    pub prefix: Prefix,
    pub desc: Vec<String>,
    pub params: Vec<Param>,
    pub returns: Vec<Return>,
    pub see: Vec<String>,
    pub usage: Option<String>,
}

impl_parse!(Func, {
    select! {
        TagType::Comment(x) => x,
        TagType::Empty => "\n".to_string()
    }
    .repeated()
    .then(select! { TagType::Param { name, ty, desc } => Param { name, ty, desc } }.repeated())
    .then(select! { TagType::Return { ty, name, desc } => Return { ty, name, desc } }.repeated())
    .then(select! { TagType::See(x) => x }.repeated())
    .then(select! { TagType::Usage(x) => x }.or_not())
    .then(select! { TagType::Func { prefix, name, scope, .. } => (name, scope, prefix.unwrap_or_default()) })
    .map(
        |(((((desc, params), returns), see), usage), (name, scope, prefix))| Self {
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
        self.prefix.right = tag;
    }

    pub fn is_public(&self, export: &str) -> bool {
        self.scope != Scope::Local && self.prefix.left == export
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
            format!("{}{}{name}", self.prefix.left, self.scope),
            format!("{}{}{}", self.prefix.right, self.scope, self.name)
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

        if !self.see.is_empty() {
            see!(f, self.see)?;
        }

        if let Some(usage) = &self.usage {
            usage!(f, usage)?;
        }

        write!(f, "")
    }
}
