use std::path::PathBuf;

#[derive(Debug)]
pub enum GUICOmmand {
    ListUnit,
    ChosenUnit(PathBuf),
    ChosenRootFolder{
        path: PathBuf,
        recurse: bool,
    },

}