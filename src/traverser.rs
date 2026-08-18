use std::{fs};
use std::path::{Path};

#[derive(Debug)]
pub enum FileNode {
    File{ name: String },
    Directory { path: String, children: Vec<FileNode> }
}

pub fn get_children(path: &Path) -> Vec<FileNode> {
    let mut children: Vec<FileNode> = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();

            // Extract the file name as an OsString
            let item_name = entry.file_name().to_string_lossy().into_owned();

            if file_path.is_file() {
                children.push(FileNode::File { name: item_name });
            } else {
                children.push(FileNode::Directory { path: file_path.to_string_lossy().into_owned(), children: (get_children(file_path.as_path())) });
            }
        }
    }

    children
}
