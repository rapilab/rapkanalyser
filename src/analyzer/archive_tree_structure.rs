use std::fs::File;
use zip::ZipArchive;
use zip::read::ZipFile;

#[derive(Clone)]
pub struct ArchiveEntry {
    path: String,
    raw_size: u64,
    download_size: u64,
}

impl ArchiveEntry {
    pub fn from_zip_file(file: ZipFile) -> ArchiveEntry {
        ArchiveEntry {
            path: file.sanitized_name().into_os_string().into_string().unwrap(),
            raw_size: file.size(),
            download_size: file.compressed_size()
        }
    }
}

pub struct ArchiveTreeStructure {

}

impl ArchiveTreeStructure {
    pub fn create(mut archive: ZipArchive<File>) -> Vec<ArchiveEntry> {
        let mut results = vec![];
        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            let entry = ArchiveEntry::from_zip_file(file);
            println!("{:?}", entry.clone().path);
            results.push(entry);
        }
        results
    }
}

