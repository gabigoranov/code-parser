use core::panic;
use std::{fs, path::Path};

use tree_sitter::{Language, Parser, Tree, TreeCursor};

use crate::classifier;

pub struct FileStats {
    pub lines: usize,
}

impl FileStats {
    fn new(lines: usize) -> Self {
        Self { lines }
    }
}

fn print_tree(cursor: &mut TreeCursor, depth: usize) {
    loop {
        let node = cursor.node();
        
        // Field name (e.g., "left", "body", "name") if available
        let field_name = cursor
            .field_name()
            .map(|f| format!("{}: ", f))
            .unwrap_or_default();

        let indent = "  ".repeat(depth);

        // Print node kind and structural flags
        if node.child_count() == 0 {
            // Leaf node (e.g., anonymous syntax tokens like "+" or named identifiers)
            println!("{}{}{}", indent, field_name, node.kind());
        } else {
            // Parent structural node
            println!("{}{}{}", indent, field_name, node.kind());
        }

        // Recurse into children
        if cursor.goto_first_child() {
            print_tree(cursor, depth + 1);
            cursor.goto_parent();
        }

        // Move to sibling at current level
        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

pub fn extract_ast(source_code: &str, lang: &Language) -> Tree {
    let mut parser = Parser::new();
    parser
        .set_language(lang)
        .expect("Error loading Rust grammar");

    let tree = parser.parse(source_code, None).unwrap();

    {
        let mut cursor = tree.walk();
        print_tree(&mut cursor, 0);
    }

    tree
}

pub fn parse_file(path: &Path) -> FileStats {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let lang_info = match classifier::get_tree_sitter_language(path) {
        Some(lang) => lang,

        None => panic!("Could not parse a file due to incompatible language!"),
    };

    extract_ast(&contents, &lang_info.language);
    FileStats::new(contents.lines().count())
}
