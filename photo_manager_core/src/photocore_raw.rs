use std::path::{Path, PathBuf};

use sysinfo::Disks;

use crate::{PhotoCore, state};

impl PhotoCore<state::Raw> {
    pub(crate) fn new() -> Self {
        Self { state: state::Raw }
    }
    pub(crate) fn get_sources(&self) -> Vec<PathBuf> {
        Disks::new_with_refreshed_list()
            .iter()
            // .filter(|disk| disk.is_removable())
            .map(|disk| disk.mount_point().to_path_buf())
            .collect()
    }
    pub(crate) fn chosen_source(self, unit: &Path) -> PhotoCore<state::SelectedSource> {
        PhotoCore {
            state: state::SelectedSource {
                source: unit.to_path_buf(),
                current: unit.to_path_buf(),
            },
        }
    }
}