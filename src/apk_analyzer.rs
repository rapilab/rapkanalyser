use std::path::PathBuf;
use crate::analyzer::archives::Archives;
use crate::sdk_constants::ANDROID_MANIFEST_XML;
use crate::manifest::android_manifest_parser::AndroidManifestParser;

pub struct ApkAnalyzer {

}

impl ApkAnalyzer {
    pub fn new() -> ApkAnalyzer {
        ApkAnalyzer {}
    }
    pub fn apk_summary(&self, apk: PathBuf) -> Vec<u8> {
        let mut manager = Archives::open(apk);
        let data = manager.get(String::from(ANDROID_MANIFEST_XML));
        AndroidManifestParser::parse(data.clone());
        data
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
        path.push("tests/resources/1.apk");

        let vec = analyzer.apk_summary(path);
        assert_eq!("<xml>
</xml>
", String::from_utf8_lossy(vec.as_ref()))
    }
}