use std::{env};
use std::path::{Path};

use code_parser::traverser::get_children;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);

    print!("{:?}", get_children(path));
}
