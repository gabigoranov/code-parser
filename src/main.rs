use std::env;
use std::path::Path;

use code_parser::traverser::{self, FileData, FileNode};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);

    let root_children = traverser::get_children(path);
    let root_files: Vec<FileData> = root_children
        .into_iter()
        .filter_map(|node| match node {
            FileNode::File(data) => Some(data),
            _ => None,
        })
        .collect();

    let gitignore_file = traverser::get_gitignore(&root_files);

    print!("{:?}", gitignore_file);
}
