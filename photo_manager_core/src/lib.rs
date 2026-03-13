//todo
// define a trait (?) for calling from worker to core
// other way to call the core
/*l'applicazione deve permettere di
selezionare unità rimovibile
selezionare cartella da usare come root
copiare foto in cartella divise in base a dato di scatto exif e un eventuale luogo (xxxx_xx_xx_luogo)
rinominare foto nella sorgente per capire le foto già copiate */

/****
 * worker molto basico, su thread separato, riceve comandi -> chiama core -> ritorna -> manda eventi
 */

use exif::DateTime;
use photo_manager_messages::FolderList;
use std::{
    fs::DirEntry,
    marker::PhantomData,
    path::{Path, PathBuf},
};



#[derive(Debug)]
pub struct PhotoFolder {
    date: DateTime,
    photo: Vec<DirEntry>,
}

trait Core {
    fn get_sources() -> Vec<PathBuf>;
    fn chosen_unit(unit: &Path) -> FolderList;
    fn chosen_folder(path: &Path, recurse: bool) -> Vec<PhotoFolder>;
}

struct Raw;
struct SelectedSource;
struct SelectedRootFolder;

struct PhotoCore<State> {
    source: Option<PathBuf>,
    root_folder: Option<PathBuf>,
    state: PhantomData<State>,
}

impl PhotoCore<Raw> {
    fn new() -> Self{
        Self {
            source: None,
            root_folder: None,
            state: std::marker::PhantomData,
        }
    }
    fn get_sources(&self) -> Vec<PathBuf>{
        todo!()
    }
    fn chosen_source(self, unit: &Path) -> PhotoCore<SelectedSource> {
        todo!()
    }
}
impl PhotoCore<SelectedSource> {
    fn get_folder(&self) -> Vec<PhotoFolder> {
        todo!()
    }
    fn chosen_folder(self, path: &Path) -> PhotoCore<SelectedRootFolder> {
        todo!()
    }
}

impl PhotoCore<SelectedRootFolder> {
    fn scan_photo(&self, recurse: bool) -> PhotoFolder {
        todo!()
    }
}
