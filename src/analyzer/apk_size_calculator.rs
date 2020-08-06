use std::path::PathBuf;
use std::fs;
use std::fs::{File};
use std::io::{Read, Write};
use flate2::write::DeflateEncoder;
use flate2::Compression;

pub trait ApkSizeCalculator {
    fn get_full_apk_download_size(&self, apk: PathBuf) -> u64;
    fn get_full_apk_raw_size(&self, apk: PathBuf) -> u64;
    fn get_download_size_per_file(&self, apk: PathBuf) -> u64;
    fn get_raw_size_per_file(&self, apk: PathBuf) -> u64;
}

pub struct GzipSizeCalculator {

}

impl GzipSizeCalculator {
    pub fn new() -> GzipSizeCalculator {
        GzipSizeCalculator {}
    }

    // todo: add verify apk file
    pub fn verify(&self, _apk: PathBuf) -> bool {
        // let mut zip = zip::ZipArchive::new(reader).unwrap();
        true
    }
}

impl ApkSizeCalculator for GzipSizeCalculator {
    fn get_full_apk_download_size(&self, apk: PathBuf) -> u64 {
        let mut buffer = vec![];
        let mut f = File::open(apk).unwrap();
        f.read_to_end(&mut buffer).unwrap();

        let mut e = DeflateEncoder::new(Vec::new(), Compression::best());
        e.write_all(buffer.as_ref()).unwrap();
        let compressed = e.finish().unwrap();
        compressed.len() as u64
    }

    fn get_full_apk_raw_size(&self, apk: PathBuf) -> u64 {
        if self.verify(apk.clone()) {
            let metadata = fs::metadata(apk);
            return match metadata {
                Ok(data) => {
                    data.len()
                },
                Err(_) => {
                    0
                },
            };
        }
        return 0
    }

    fn get_download_size_per_file(&self, _apk: PathBuf) -> u64 {
        unimplemented!()
    }

    fn get_raw_size_per_file(&self, _apk: PathBuf) -> u64 {
        unimplemented!()
    }
}
