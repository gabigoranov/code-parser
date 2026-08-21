use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FileData {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct DirectoryData {
    pub path: PathBuf,
    pub children: Vec<FileNode>,
}

#[derive(Debug)]
pub enum FileNode {
    File(FileData),
    Directory(DirectoryData),
}

pub fn get_children(path: &Path, gitignore_file: &gitignore::File) -> Vec<FileNode> {
    let mut children: Vec<FileNode> = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();

            // skip hiden files
            if file_path.starts_with(".") { continue }
            if let Ok(is_excluded) = gitignore_file.is_excluded(&file_path) && is_excluded {
                continue
            }
            
            // Extract the file name as an OsString
            let item_name = entry.file_name().to_string_lossy().into_owned();

            if file_path.is_file() {
                children.push(FileNode::File(FileData {
                    name: item_name,
                    path: file_path.clone(),
                }));
            } else {
                children.push(FileNode::Directory(DirectoryData {
                    path: file_path.as_path().to_owned(),
                    children: (get_children(file_path.as_path(), gitignore_file)),
                }));
            }
        }
    }

    children
}
