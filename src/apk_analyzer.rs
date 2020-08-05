use std::path::PathBuf;
use crate::analyzer::archives::Archives;
use crate::sdk_constants::ANDROID_MANIFEST_XML;
use crate::manifest::android_manifest_parser::AndroidManifestParser;
use crate::manifest::manifest_data::ManifestData;
use crate::binary_xml::binary_xml_parser::BinaryXmlParser;
use crate::analyzer::apk_size_calculator::{GzipSizeCalculator, ApkSizeCalculator};

pub struct ApkAnalyzer {

}

pub fn get_manifest_data(data: Vec<u8>) {
    BinaryXmlParser::decode_xml(data);
}

impl ApkAnalyzer {
    pub fn new() -> ApkAnalyzer {
        ApkAnalyzer {}
    }
    pub fn file_size(&self, apk: PathBuf) -> u64 {
        let calculator = GzipSizeCalculator::new();
        calculator.get_full_apk_raw_size(apk)
    }
    pub fn download_size(&self, apk: PathBuf) -> u64 {
        let calculator = GzipSizeCalculator::new();
        calculator.get_full_apk_download_size(apk)
    }
    pub fn apk_summary(&self, apk: PathBuf) -> ManifestData {
        let mut manager = Archives::open(apk);
        let data = manager.get(String::from(ANDROID_MANIFEST_XML));

        get_manifest_data(data.clone());

        *AndroidManifestParser::parse(data.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::apk_analyzer::ApkAnalyzer;
    use std::path::PathBuf;

    #[test]
    fn should_identify_application_name_from_apk() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let vec = analyzer.apk_summary(path);
        // assert_eq!("com.example.android.multiproject", vec.m_package)
    }

    #[test]
    fn should_get_apk_size() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let size = analyzer.file_size(path);
        assert_eq!(48196, size)
    }

    #[test]
    fn should_get_download_size() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let size = analyzer.download_size(path);
        assert_eq!(39591, size)
    }
}