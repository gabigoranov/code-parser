use std::env;
use std::path::Path;

use code_parser::traverser::{Traverser};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);
    let gitignore_path = path.join(".gitignore");
    let gitignore_file = gitignore::File::new(&gitignore_path).expect("Please enter a directory with a valid .gitignore file!");

    let traverser = Traverser::new(path.to_owned(), gitignore_file);
    let root_children = traverser.get_children();

    print!("{root_children:#?}");
}
