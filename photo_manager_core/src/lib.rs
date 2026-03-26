/*l'applicazione deve permettere di
selezionare unità rimovibile
selezionare cartella da usare come root
copiare foto in cartella divise in base a dato di scatto exif e un eventuale luogo (xxxx_xx_xx_luogo)
rinominare foto nella sorgente per capire le foto già copiate */

use std::path::PathBuf;

mod state {
    use std::path::PathBuf;
    pub(crate) struct Raw;
    pub(crate) struct SelectedSource {
        pub(crate) source: PathBuf,
        pub(crate) current: PathBuf,
    }
    pub(crate) struct SelectedRootFolder {
        pub(crate) root: PathBuf,
    }
}

struct PhotoCore<State> {
    state: State,
}

mod photocore_raw;
mod photocore_selected_source;
mod photocore_selected_root_folder;

#[test]
fn test1() {
    let core = PhotoCore::new();
    let sources = core.get_sources();
    println!("{sources:?}");
    let directories = core.chosen_source(&sources[1]).list_dirs();
    println!("{directories:?}")

    let path = photocore_selected_source::DirType::Child(PathBuf::new());
}
