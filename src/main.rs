use std::fs::read_to_string;

use tree_sitter::{Node, Query, QueryCursor};

use lemmy_help::LemmyHelp;

// const QUERY: &str = r#"
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

// const QUERY: &str = r#"
// (
//     (comment)
//     [
//         (function_declaration
//             (dot_index_expression) @func)
//         (assignment_statement
//             (variable_list) @func
//             (expression_list
//                 value: (function_definition)))
//         (assignment_statement
//             (variable_list) @expr
//             (expression_list
//                 value: [(table_constructor)]))
//     ]
// )
// (
//     (comment)+ @doc
// )
// "#;

fn get_node_text(node: Node, src: &[u8]) -> String {
    let mut text = node.utf8_text(src).unwrap_or("").to_string();
    text.push('\n');
    text
}

fn is_exported(node: &Node) -> bool {
    node.kind() == "dot_index_expression"
}

fn what_next(node: Node, source: &[u8]) -> Option<String> {
    match node.next_named_sibling() {
        Some(x) if x.kind() == "function_declaration" => {
            let name = x.named_child(0).expect("missing function name!");
            if is_exported(&name) {
                let name = name
                    .utf8_text(source)
                    .expect("Unable to get the function name!");
                return Some(format!("---@name {name}\n"));
            };

            None
        }
        Some(x) if x.kind() == "assignment_statement" => {
            let name = x.named_child(0).expect("missing assigment name!");

            if !is_exported(&name.named_child(0).expect("WTF")) {
                return None;
            };

            let name = name
                .utf8_text(source)
                .expect("Unable to get the export name!");

            return Some(format!("---@name {name}\n"));
        }
        _ => None,
    }
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
        let last_src_node = match ele.captures.last() {
            Some(x) => what_next(x.node, src_bytes),
            _ => None,
        };

        let mut doc = ele
            .captures
            .iter()
            .map(|x| get_node_text(x.node, src_bytes))
            .collect::<String>();

        if let Some(last) = last_src_node {
            doc.push_str(&last)
        }

        // dbg!(&doc);
        dbg!(LemmyHelp::parse(&doc).unwrap());
        // print!("{}", LemmyHelp::parse(&doc).unwrap());
    }
}
