use std::fs;
use std::path::{Path, PathBuf};

use crate::{classifier, parser};

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

pub struct Traverser<'a> {
    root: PathBuf,
    gitignore_file: gitignore::File<'a>,
}

impl<'a> Traverser<'a> {
    pub fn new(root: PathBuf, gitignore_file: gitignore::File<'a>) -> Self {
        Self {
            root,
            gitignore_file,
        }
    }

    pub fn get_children(&self) -> Vec<FileNode> {
        self.traverse_path(&self.root)
    }

    fn traverse_path(&self, path: &Path) -> Vec<FileNode> {
        let mut children: Vec<FileNode> = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                let file_name = entry.file_name().to_string_lossy().into_owned();

                if self.unworthy(&file_name, &file_path) {
                    continue;
                }

                let node = match file_path.is_file() {
                    true => {
                        let language = match classifier::get_tree_sitter_language(&file_path) {
                            Some(lang) => lang,

                            None => continue,
                        };

                        parser::parse_file(&file_path);
                        FileNode::File(FileData {
                            name: file_name,
                            path: file_path,
                        })
                    },

                    false => {
                        FileNode::Directory(DirectoryData {
                            children: self.traverse_path(&file_path),
                            path: file_path,
                        })
                    }
                };

                children.push(node);
            }
        }

        children
    }

    fn unworthy(&self, file_name: &str, file_path: &Path) -> bool {
        let is_ignored = self.gitignore_file.is_excluded(file_path).unwrap_or(false);

        if is_ignored || file_name.starts_with('.') {
            println!("{file_path:?} is unworthy");
            return true;
        }

        false
    }
}
