use std::{fs, path::Path};

pub struct FileStats {
    pub lines: usize,
}

impl FileStats {
    fn new(lines: usize) -> Self {
        Self {
            lines
        }
    }
}

// pub fn extract_ast(source_code: &String) -> Tree {
    // let mut parser = Parser::new();  
    // parser.set_language(&tree_sitter_rust::LANGUAGE.into()).expect("Error loading Rust grammar");

    // let tree = parser.parse(source_code, None).unwrap();
    // let root_node = tree.root_node();

    // println!("{:?}", tree);
    // tree
// }

pub fn parse_file(path: &Path) -> FileStats {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    print!("{}", path.to_string_lossy());
    // extract_ast(&contents);
    FileStats::new(contents.lines().count())
}
