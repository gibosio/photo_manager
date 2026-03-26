use photo_manager_messages::PhotoFolder;

use crate::{PhotoCore, state};


impl PhotoCore<state::SelectedRootFolder> {
    fn scan_photo(&self, recurse: bool) -> Vec<PhotoFolder> {
        // fn get_all_photo(path: &Path, recurse: bool) -> Vec<PathBuf>{
        //     let a: fs::ReadDir = fs::read_dir(path).unwrap();
        //     let b: Vec<DirEntry> = a.flatten().collect();
        //     let files: Vec<FileType> = b.into_iter().map(|e|e.file_type()).flatten().collect();
        //     let only_files: Vec<FileType> = files.into_iter().filter(|t|t.is_file()).collect();
        //     let only_photos = only_files
        //         .into_iter()
        //         .map(|f|)

        //     todo!()
        // }
        //get all photo
        //extract all the exif data information
        //construct PhotoFolder items
        // maybe use hashmap or hashset for having a single datetime

        todo!()
    }
}
