//todo
// define a trait (?) for calling from worker to core
// other way to call the core
/*l'applicazione deve permettere di
selezionare unità rimovibile
selezionare cartella da usare come root
copiare foto in cartella divise in base a dato di scatto exif e un eventuale luogo (xxxx_xx_xx_luogo)
rinominare foto nella sorgente per capire le foto già copiate */

use photo_manager_messages::{FolderList, PhotoFolder};
use std::path::{Path, PathBuf};
use sysinfo::{Disks, System};

// trait Core {
//     fn get_sources() -> Vec<PathBuf>;
//     fn chosen_unit(unit: &Path) -> FolderList;
//     fn chosen_folder(path: &Path, recurse: bool) -> Vec<PhotoFolder>;
// }

struct Raw;
struct SelectedSource {
    source: PathBuf,
}
struct SelectedRootFolder {
    source: PathBuf,
    root: PathBuf,
}

struct PhotoCore<State> {
    state: State,
}

impl PhotoCore<Raw> {
    fn new() -> Self {
        Self { state: Raw }
    }
    fn get_sources(&self) -> Vec<PathBuf> {
        let disks = Disks::new_with_refreshed_list();
        disks
            .iter()
            .filter(|disk|disk.is_removable())
            .map(|disk|disk.mount_point().to_path_buf())
            .collect()
    }
    fn chosen_source(self, unit: &Path) -> PhotoCore<SelectedSource> {
        PhotoCore {
            state: SelectedSource {
                source: unit.to_path_buf(),
            },
        }
    }
}
impl PhotoCore<SelectedSource> {
    fn get_folder(&self) -> Vec<FolderList> {
        todo!()
    }
    fn chosen_folder(self, path: &Path) -> PhotoCore<SelectedRootFolder> {
        PhotoCore {
            state: SelectedRootFolder {
                source: self.state.source,
                root: path.to_path_buf(),
            },
        }
    }
}

impl PhotoCore<SelectedRootFolder> {
    fn scan_photo(&self, recurse: bool) -> Vec<PhotoFolder> {
        todo!()
    }
}

#[test]
fn test1(){
    let core = PhotoCore::new();
    let sources = core.get_sources();
    println!("{sources:?}");

}