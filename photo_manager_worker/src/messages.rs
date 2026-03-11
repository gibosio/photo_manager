use std::path::PathBuf;

#[derive(Debug)]
pub enum GUICommand {
    ListUnit,
    ChosenUnit(PathBuf),
    ChosenRootFolder{
        path: PathBuf,
        recurse: bool,
    },

}