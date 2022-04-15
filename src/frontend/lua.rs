use chumsky::{
    prelude::{choice, filter, just, take_until, Simple},
    text::{keyword, newline, TextParser},
    Parser,
};

#[derive(Debug)]
pub struct Lua;

impl Lua {
    pub fn parse(src: &str) -> Result<String, Vec<Simple<char>>> {
        Ok(Self::lex(src)?.join("\n"))
    }

    // TODO: support ignoring `---@private`
    pub fn lex(src: &str) -> Result<Vec<String>, Vec<Simple<char>>> {
        let func = keyword("function")
            .padded()
            .ignore_then(filter(|x| *x != '(').repeated().collect());

        let local = keyword("local").padded();

        let node = choice((
            just("---")
                .then(filter(|c| *c != '\n').repeated().collect::<String>())
                .map(|(s, x)| format!("{s}{x}")),
            func.clone().map(|x: String| format!("---@func {x} public")),
            local
                .ignore_then(func)
                .map(|x| format!("---@func {x} private")),
        ))
        .map(Some);

        let misc = take_until(newline()).to(None);

        choice((node.padded(), misc))
            .repeated()
            .collect::<Vec<Option<String>>>()
            .flatten()
            .parse(src)
    }
}
