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

// Extract the main .gitignore in the project root dir to not parse hidden items
pub fn get_gitignore<'a>(root_children: &'a [FileData]) -> Option<gitignore::File<'a>> {
    for FileData { name, path } in root_children.iter() {
        if name == ".gitignore" {
            return gitignore::File::new(path).ok()
        }
    }

    None
}

pub fn get_children(path: &Path) -> Vec<FileNode> {
    let mut children: Vec<FileNode> = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();

            // skip hiden files
            if file_path.starts_with(".") { continue }
            
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
                    children: (get_children(file_path.as_path())),
                }));
            }
        }
    }

    children
}
