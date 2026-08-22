use std::path::Path;
use tree_sitter::Language;
use tree_sitter_language_pack::{detect_language_from_path, get_language};

#[derive(Debug)]
pub struct LanguageInfo {
    pub name: String,
    pub language: Language,
}

// prematurely returns None if it can not parse the file a.k.a it is not a code file or it is a dir
pub fn get_tree_sitter_language(path: &Path) -> Option<LanguageInfo> {
    let language_name = detect_language_from_path(&path.to_string_lossy())?;

    Some(LanguageInfo {
        name: language_name.to_owned(),
        language: get_language(language_name).ok()?,
    })
}
