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

pub fn parse_file(path: &Path) -> FileStats {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    FileStats::new(contents.lines().count())
}
