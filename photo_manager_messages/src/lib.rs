use std::path::PathBuf;

#[derive(Debug)]
pub enum GUICommand {
    ListSources,
    ChosenSource(PathBuf),
    ChosenRootFolder{
        path: PathBuf,
        recurse: bool,
    },

}
#[derive(Debug)]
pub enum GuiEvent {
    SourcesList(Vec<PathBuf>),
    FoldersList(FolderList),
    
}

#[derive(Debug)]
pub struct FolderList {
    pub path: PathBuf,
    pub subfolders: Vec<FolderList>,
}