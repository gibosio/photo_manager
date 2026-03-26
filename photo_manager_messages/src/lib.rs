use exif::DateTime;
use std::fs::DirEntry;
use std::path::PathBuf;

#[derive(Debug)]
pub enum GUICommand {
    ListSources,
    ChosenSource(PathBuf),
    ChosenRootFolder { path: PathBuf, recurse: bool },
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
#[derive(Debug)]
pub struct PhotoFolder {
    date: CustomDatetime,
    photo: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct CustomDatetime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl From<DateTime> for CustomDatetime {
    fn from(exif_datetime: DateTime) -> Self {
        Self {
            year: exif_datetime.year,
            month: exif_datetime.month,
            day: exif_datetime.day,
        }
    }
}
