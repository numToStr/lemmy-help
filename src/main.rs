use std::fs::read_to_string;
mod common;
mod parsers;

use chumsky::Parser;
use tree_sitter::{Query, QueryCursor};

use crate::parsers::Node;

// const Q: &str = r#"
// (return_statement (expression_list (identifier) @export))
// (
//     (dot_index_expression (identifier) @ident)
//     (#not-eq? @ident @export)
// )
// "#;

const Q: &str = r#"
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

fn main() {
    let source = read_to_string("src/fixtures/test.lua").unwrap();

    let mut parser = tree_sitter::Parser::new();
    let lang = tree_sitter_lua::language();

    parser
        .set_language(lang)
        .expect("Error loading lua grammar");

    let tree = parser.parse(&source, None).unwrap();
    let query = Query::new(lang, Q).unwrap();

    let mut cursor = QueryCursor::new();
    let src_bytes = source.as_bytes();

    for ele in cursor.matches(&query, tree.root_node(), src_bytes) {
        let doc = ele
            .captures
            .iter()
            .map(|x| {
                let x = x
                    .node
                    .utf8_text(src_bytes)
                    .unwrap_or("")
                    .trim_start_matches("--- ")
                    .trim_start_matches("---");

                let mut n = String::with_capacity(x.len() + 1);

                n.push_str(x);
                n.push('\n');

                n
            })
            .collect::<String>();

        dbg!(Node::parse().parse(doc).unwrap());
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
