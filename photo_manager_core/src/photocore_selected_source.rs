use std::{
    fs,
    path::{Path, PathBuf},
};

use photo_manager_messages::FolderList;

use crate::{PhotoCore, state};

impl PhotoCore<state::SelectedSource> {
    fn get_folder(&self, recurse: bool) -> FolderList {
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
        fn build_folder_tree(path: &Path, recurse: bool) -> FolderList {
            let subfolders = fs::read_dir(path)
                .into_iter()
                .flatten()
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|entry| entry.is_dir())
                .map(|p| {
                    if recurse {
                        build_folder_tree(&p, recurse)
                    } else {
                        FolderList {
                            path: p.to_path_buf(),
                            subfolders: vec![],
                        }
                    }
                })
                .collect();
            FolderList {
                path: path.to_path_buf(),
                subfolders,
            }
        }
        build_folder_tree(&self.state.source, recurse)
    }
    pub(crate) fn list_dirs(&self) -> Vec<PathBuf> {
        let parent = self.state.current.parent();
        let dirs = fs::read_dir(&self.state.current)
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|entry| entry.is_dir())
            .collect::<Vec<PathBuf>>();
        std::iter::once(self.state.current.to_path_buf())
            .chain(parent.into_iter().map(|p| p.to_path_buf()))
            .chain(dirs)
            .collect()
    }
    fn chosen_folder(self, path: &Path) -> PhotoCore<state::SelectedRootFolder> {
        PhotoCore {
            state: state::SelectedRootFolder {
                root: path.to_path_buf(),
            },
        }
    }
}
