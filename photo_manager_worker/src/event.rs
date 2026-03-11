use std::path::PathBuf;

use crate::helper::FolderList;

#[derive(Debug)]
pub enum GuiEvent {
    UnitsList(Vec<PathBuf>),
    FoldersList(FolderList)
}