use std::fs::read_to_string;

use lemmy_help::{frontend::Lua, LemmyHelp};

fn main() {
    let source = read_to_string("src/fixtures/test.lua").unwrap();
    let source = Lua::parse(&source).unwrap();
    let nodes = LemmyHelp::parse(&source).unwrap();

    dbg!(&nodes);
    print!("{nodes}");
}
