use chumsky::{
    prelude::{choice, end, filter, just, take_until, Simple},
    text::{ident, keyword, newline, TextParser},
    Parser,
};

#[derive(Debug)]
pub struct Lua;

impl Lua {
    pub fn parse(src: &str) -> Result<String, Vec<Simple<char>>> {
        Ok(Self::lex(src)?.join("\n"))
    }

    // TODO: support ignoring via `---@private`
    pub fn lex(src: &str) -> Result<Vec<String>, Vec<Simple<char>>> {
        let dotted = ident()
            .then(just('.').or(just(':')))
            .then(ident())
            .map(|((m, k), f)| format!("{m}{k}{f}"));

        let expr = dotted.padded().then_ignore(just('='));

        let func = keyword("function").padded();

        let local = keyword("local").padded();

        let node = choice((
            just("---")
                .then(filter(|c| *c != '\n').repeated().collect::<String>())
                .map(|(s, x)| format!("{s}{x}")),
            local.ignore_then(choice((
                func.clone()
                    .ignore_then(ident())
                    .map(|x| format!("---@func {x} private")),
                ident()
                    .padded()
                    .then_ignore(just('='))
                    .map(|x| format!("---@expr {x} private")),
            ))),
            func.clone()
                .ignore_then(dotted)
                .map(|x| format!("---@func {x} public")),
            choice((
                expr.then_ignore(func)
                    .map(|x| format!("---@func {x} public")),
                expr.map(|x| format!("---@expr {x} public")),
            )),
            keyword("return")
                .ignore_then(ident().padded())
                .then_ignore(end())
                .map(|x| format!("---@export {x}")),
        ))
        .map(Some);

        let misc = take_until(newline()).to(None);

        choice((node.padded(), misc))
            .repeated()
            .flatten()
            .parse(src)
    }
}
