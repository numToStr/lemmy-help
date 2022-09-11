mod divider;
pub use divider::*;
mod alias;
pub use alias::*;
mod brief;
pub use brief::*;
mod class;
pub use class::*;
mod func;
pub use func::*;
mod tag;
pub use tag::*;
mod r#type;
pub use r#type::*;
mod module;
pub use module::*;
mod see;
pub use see::*;
mod usage;
pub use usage::*;

#[derive(Debug, Default, Clone)]
pub struct Prefix {
    pub left: Option<String>,
    pub right: Option<String>,
}
