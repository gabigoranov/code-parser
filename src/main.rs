use std::env;
use std::path::Path;

use code_parser::traverser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);
    let gitignore_path = path.join(".gitignore");
    let gitignore_file = gitignore::File::new(&gitignore_path);

    print!("{:?}", gitignore_file);

    let root_children = traverser::get_children(
        path,
        &gitignore_file.expect("Please enter a directory with a valid .gitignore file!"),
    );

    print!("{root_children:?}");
}
