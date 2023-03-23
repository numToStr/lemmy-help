mod node;
pub use node::*;
mod tags;
pub use tags::*;

use crate::lexer::Token;
use chumsky::{input::SpannedInput, prelude::Rich, span::SimpleSpan, Parser};

pub type ParserInput<'tokens, 'src> =
    SpannedInput<Token<'src>, SimpleSpan, &'tokens [(Token<'src>, SimpleSpan)]>;

pub type ParserErr<'tokens, 'src> = chumsky::extra::Err<Rich<'tokens, Token<'src>, SimpleSpan>>;

pub trait LemmyParser<'tokens, 'src: 'tokens, O>:
    Parser<'tokens, ParserInput<'tokens, 'src>, O, ParserErr<'tokens, 'src>> + Clone
{
}

impl<'tokens, 'src, O, P> LemmyParser<'tokens, 'src, O> for P
where
    'src: 'tokens,
    P: Parser<'tokens, ParserInput<'tokens, 'src>, O, ParserErr<'tokens, 'src>> + Clone,
{
}
