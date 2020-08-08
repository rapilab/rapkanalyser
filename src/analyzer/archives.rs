use crate::analyzer::archive_manager::ArchiveManager;
use std::path::PathBuf;

pub struct Archives {}

impl Archives {
    pub fn open(apk_path: PathBuf) -> ArchiveManager {
        let file = std::fs::File::open(&apk_path).unwrap();
        let archives = zip::ZipArchive::new(file).unwrap();
        ArchiveManager::new(archives)
    }
}
