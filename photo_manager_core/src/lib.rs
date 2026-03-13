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
use sysinfo::Disks;

mod state {
    use std::path::PathBuf;
    pub(crate) struct Raw;
    pub(crate) struct SelectedSource {
        pub(crate) source: PathBuf,
    }
    pub(crate) struct SelectedRootFolder {
        pub(crate) source: PathBuf,
        pub(crate) root: PathBuf,
    }
}

struct PhotoCore<State> {
    state: State,
}

impl PhotoCore<state::Raw> {
    fn new() -> Self {
        Self { state: state::Raw }
    }
    fn get_sources(&self) -> Vec<PathBuf> {
        Disks::new_with_refreshed_list()
            .iter()
            .filter(|disk| disk.is_removable())
            .map(|disk| disk.mount_point().to_path_buf())
            .collect()
    }
    fn chosen_source(self, unit: &Path) -> PhotoCore<state::SelectedSource> {
        PhotoCore {
            state: state::SelectedSource {
                source: unit.to_path_buf(),
            },
        }
    }
}
impl PhotoCore<state::SelectedSource> {
    fn get_folder(&self) -> FolderList {
        /// recursive function for making a tree of directories
        fn build_folder_tree_imperative(path: &Path) -> FolderList {
            let mut subfolders = vec![];
            if let Ok(directories) = fs::read_dir(path) {
                for directory_path in directories.flatten().map(|d| d.path()) {
                    if directory_path.is_dir() {
                        subfolders.push(build_folder_tree_imperative(&directory_path));
                    }
                }
            }
            FolderList {
                path: path.to_path_buf(),
                subfolders,
            }
        }
        fn build_folder_tree(path: &Path) -> FolderList {
            let subfolders = fs::read_dir(path)
                .into_iter()
                .flatten()
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|entry| entry.is_dir())
                .map(|p| build_folder_tree(&p))
                .collect();
            FolderList {
                path: path.to_path_buf(),
                subfolders,
            }
        }
        build_folder_tree(&self.state.source)
    }
    fn chosen_folder(self, path: &Path) -> PhotoCore<state::SelectedRootFolder> {
        PhotoCore {
            state: state::SelectedRootFolder {
                source: self.state.source,
                root: path.to_path_buf(),
            },
        }
    }
}

impl PhotoCore<state::SelectedRootFolder> {
    fn scan_photo(&self, recurse: bool) -> Vec<PhotoFolder> {
        todo!()
    }
}

#[test]
fn test1() {
    let core = PhotoCore::new();
    let sources = core.get_sources();
    println!("{sources:?}");
    let directories = core.chosen_source(&sources[0]).get_folder();
    println!("{directories:?}")
}
