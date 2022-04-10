use std::fs::read_to_string;
mod common;
mod parsers;

use tree_sitter::{Node, Query, QueryCursor};

use crate::parsers::LemmyHelp;

// const Q: &str = r#"
// (return_statement (expression_list (identifier) @export))
// (
//     (dot_index_expression (identifier) @ident)
//     (#not-eq? @ident @export)
// )
// "#;

const QUERY: &str = r#"
(
    (comment)+ @doc
    (#not-eq? @doc "---@private")
)
"#;

// const Q: &str = r#"
// (
//     (comment)+ @doc
// )
// (
//     (comment)+
//     [
//         (function_declaration
//             (dot_index_expression) @block)
//         (assignment_statement
//             (variable_list) @block)
//     ]?
// )
// "#;

fn convert_node(node: Node, src: &[u8]) -> String {
    let old = node
        .utf8_text(src)
        .unwrap_or("")
        .trim_start_matches("--- ")
        .trim_start_matches("---");

    let mut text = String::with_capacity(old.len() + 1);

    text.push_str(old);
    text.push('\n');

    text
}

fn main() {
    let mut parser = tree_sitter::Parser::new();
    let lang = tree_sitter_lua::language();

    parser
        .set_language(lang)
        .expect("Error loading lua grammar");

    let query = Query::new(lang, QUERY).unwrap();
    let mut cursor = QueryCursor::new();

    let source = read_to_string("src/fixtures/test.lua").unwrap();
    let src_bytes = source.as_bytes();
    let tree = parser.parse(src_bytes, None).unwrap();

    for ele in cursor.matches(&query, tree.root_node(), src_bytes) {
        let doc = ele
            .captures
            .iter()
            .map(|x| convert_node(x.node, src_bytes))
            .collect::<String>();

        dbg!(LemmyHelp::parse(&doc).unwrap());

        // dbg!(ele
        //     .captures
        //     .iter()
        //     .next()
        //     .unwrap()
        //     .node
        //     .utf8_text(source.as_bytes()));
        // for capture in ele.captures {
        //     dbg!(capture);
        //     // dbg!(capture.node.utf8_text(code.as_bytes()).unwrap());
        //     // dbg!(capture
        //     //     .node
        //     //     .next_sibling()
        //     //     .unwrap()
        //     //     .utf8_text(code.as_bytes())
        //     //     .unwrap());
        // }
    }
}
