use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FolderList {
    pub path: PathBuf,
    pub subfolders: Vec<FolderList>
}