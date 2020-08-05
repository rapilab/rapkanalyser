use std::path::PathBuf;
use std::fs;

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

    pub fn verify(&self, apk: PathBuf) {

    }
}

impl ApkSizeCalculator for GzipSizeCalculator {
    fn get_full_apk_download_size(&self, apk: PathBuf) -> u64 {
        unimplemented!()
    }

    fn get_full_apk_raw_size(&self, apk: PathBuf) -> u64 {
        self.verify(apk.clone());
        let metadata = fs::metadata(apk);
        metadata.unwrap().len()
    }

    fn get_download_size_per_file(&self, apk: PathBuf) -> u64 {
        unimplemented!()
    }

    fn get_raw_size_per_file(&self, apk: PathBuf) -> u64 {
        unimplemented!()
    }
}
