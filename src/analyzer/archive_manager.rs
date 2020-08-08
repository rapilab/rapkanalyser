use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

pub struct ArchiveFile {}

pub struct ArchiveManager {
    pub files: ZipArchive<File>,
}

impl ArchiveManager {
    pub fn new(files: ZipArchive<File>) -> ArchiveManager {
        ArchiveManager { files }
    }

    pub fn get(&mut self, name: String) -> Vec<u8> {
        let mut resources_content = Vec::new();

        &self
            .files
            .by_name(name.as_ref())
            .unwrap()
            .read_to_end(&mut resources_content)
            .unwrap();

        resources_content
    }
}
