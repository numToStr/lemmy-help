use std::fmt::Display;

use chumsky::select;

use crate::{child_table, impl_parse, section, Object, TagType};

#[derive(Debug, Clone)]
pub struct Alias(Object);

impl_parse!(Alias, {
    select! {TagType::Alias(x) => Self(x)}
});

impl Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = &self.0;

        let ty = child_table!("Type: ~", [[&a.ty]]);

        let alias = section!(
            a.name.as_str(),
            a.name.as_str(),
            a.desc.as_deref().unwrap_or_default(),
            vec![ty.to_string()]
        );

        f.write_str(alias.to_string().as_str())
    }
}
