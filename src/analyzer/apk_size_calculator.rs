use std::path::PathBuf;
use std::fs;
use std::fs::Metadata;
use std::io::Error;

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
    pub fn verify(&self, apk: PathBuf) -> bool {
        // let mut zip = zip::ZipArchive::new(reader).unwrap();
        true
    }
}

impl ApkSizeCalculator for GzipSizeCalculator {
    fn get_full_apk_download_size(&self, apk: PathBuf) -> u64 {
        unimplemented!()
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

    fn get_download_size_per_file(&self, apk: PathBuf) -> u64 {
        unimplemented!()
    }

    fn get_raw_size_per_file(&self, apk: PathBuf) -> u64 {
        unimplemented!()
    }
}
