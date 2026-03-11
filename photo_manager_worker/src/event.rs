use std::path::PathBuf;

use crate::helper::FolderList;

#[derive(Debug)]
pub enum CoreEvent {
    UnitsList(Vec<PathBuf>),
    FoldersList(FolderList)
}