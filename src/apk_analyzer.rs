use std::path::PathBuf;
use crate::analyzer::archives::Archives;
use crate::sdk_constants::{ANDROID_MANIFEST_XML, RESOURCES_ARSC};
use crate::manifest::android_manifest_parser::AndroidManifestParser;
use crate::manifest::manifest_data::ManifestData;
use crate::binary_xml::binary_xml_parser::BinaryXmlParser;
use crate::analyzer::apk_size_calculator::{GzipSizeCalculator, ApkSizeCalculator};

use crate::analyzer::archive_tree_structure::{ArchiveTreeStructure, ArchiveEntry};

use dex::{DexReader, Dex};
use crate::analyzer::dex::dex_file_stats::DexFileStats;
use std::io::{Read, Write, Cursor};
use std::fs::File;
use tempfile::tempdir;
use memmap::Mmap;
use crate::analyzer::dex::package_tree_creator::PackageTreeCreator;
use crate::analyzer::dex::dex_package_node::DexPackageNode;
use crate::analyzer::dex::DexElementNode;
use crate::arsc::binary_resource_file::BinaryResourceFile;
use failure::Error;

fn get_all_dex_from_apk(apk: PathBuf) -> Vec<Dex<Mmap>> {
    let mut archive = Archives::open(apk).files;
    let mut dex_results = vec![];

    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i).unwrap();
        let file_name = zip_file.name();
        if file_name.ends_with(".dex") {
            let mut buffer = Vec::new();
            let dir = tempdir().unwrap();
            let file_path = dir.path().join(file_name);
            let mut file = File::create(file_path.clone()).unwrap();

            zip_file.read_to_end(&mut buffer);
            file.write(&*buffer);

            let result = DexReader::from_file(file_path);
            match result {
                Ok(data) => {
                    dex_results.push(data);
                }
                Err(_) => {}
            }
        }
    };

    dex_results
}

pub struct ApkAnalyzer {}

impl ApkAnalyzer {
    pub fn new() -> ApkAnalyzer {
        ApkAnalyzer {}
    }
    pub fn apk_file_size(&self, apk: PathBuf) -> u64 {
        let calculator = GzipSizeCalculator::new();
        calculator.get_full_apk_raw_size(apk)
    }

    pub fn apk_download_size(&self, apk: PathBuf) -> u64 {
        let calculator = GzipSizeCalculator::new();
        calculator.get_full_apk_download_size(apk)
    }

    pub fn apk_summary(&self, apk: PathBuf) -> ManifestData {
        let result = self.manifest_print(apk);
        let manifest = AndroidManifestParser::parse(Vec::from(result.as_bytes()));
        *manifest
    }

    pub fn files_list(&self, apk: PathBuf) -> Vec<ArchiveEntry> {
        let manager = Archives::open(apk);
        let mut vec = ArchiveTreeStructure::create(manager.files);
        vec.sort_by(|a, b| a.path.cmp(&b.path));
        vec
    }

    pub fn file_cat(&self, apk: PathBuf, name: String) -> String {
        let mut manager = Archives::open(apk);
        let data = manager.get(String::from(name));
        String::from(String::from_utf8_lossy(&*data))
    }

    pub fn manifest_print(&self, apk: PathBuf) -> String {
        let mut manager = Archives::open(apk);
        let data = manager.get(String::from(ANDROID_MANIFEST_XML));

        let result = BinaryXmlParser::decode_xml(data).unwrap();
        result
    }

    pub fn dex_list(&self, apk: PathBuf) -> Vec<ArchiveEntry> {
        let mut archive = Archives::open(apk).files;
        let mut results = vec![];
        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            if file.name().ends_with(".dex") {
                let entry = ArchiveEntry::from_zip_file(file);
                results.push(entry);
            }
        }
        results
    }

    pub fn dex_references(&self, apk: PathBuf) -> Vec<DexFileStats> {
        let dexes = get_all_dex_from_apk(apk);
        let mut files_stats: Vec<DexFileStats> = vec![];
        for x in dexes {
            files_stats.push(DexFileStats::create(x))
        }

        files_stats
    }

    pub fn dump_tree(&self, node: DexElementNode, parent_info: String) -> String {
        let mut node_type: &str = "";
        let some = "d ";

        let mut node_name: String = String::from("");

        match node.clone() {
            DexElementNode::DexPackage(pkg) => {
                node_type = "P ";
                node_name = pkg.name;
            }
            DexElementNode::DexClass(clz) => {
                node_type = "C ";
                node_name = clz.name;
            }
            DexElementNode::DexMethod(method) => {
                node_type = "M ";
                node_name = method.name;
            }
            DexElementNode::DexField(field) => {
                node_type = "F ";
                node_name = field.name;
            }
        }

        let string = format!("{} {} {} {}", node_type, some, parent_info, node_name);
        println!("{}", string);
        match node.clone() {
            DexElementNode::DexPackage(pkg) => {
                for class_node in pkg.class_nodes {
                    let info =  format!("{}.{}", pkg.name, class_node.name);
                    self.dump_tree(DexElementNode::DexClass(
                        class_node.clone()), String::from(info)
                    );
                }
            }
            DexElementNode::DexClass(clz) => {
                for child in clz.child {
                    // self.dump_tree(DexElementNode::DexMethod(method.clone()));
                    match child {
                        DexElementNode::DexMethod(method) => {
                            let info =  format!("{}.{}", parent_info, clz.name);
                            self.dump_tree(DexElementNode::DexMethod(
                                method.clone()), String::from(info)
                            );
                        }
                        DexElementNode::DexField(field) => {
                            let info =  format!("{}.{}", parent_info, clz.name);
                            self.dump_tree(DexElementNode::DexField(
                                field.clone()), String::from(info)
                            );
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        string
    }
    pub fn dex_packages(&self, apk: PathBuf) -> DexPackageNode {
        let dexes = get_all_dex_from_apk(apk);
        let creator = PackageTreeCreator::new();
        let node = creator.construct_package_tree(dexes);
        self.dump_tree(DexElementNode::DexPackage(node.clone()), String::from(""));
        node
    }

    pub fn res_package(&self, apk: PathBuf) -> Result<String, Error> {
        let mut manager = Archives::open(apk);
        let data = manager.get(String::from(RESOURCES_ARSC));

        let file = BinaryResourceFile::new();
        let cursor: Cursor<&[u8]> = Cursor::new(&*data);
        file.decode_arsc(cursor)
    }
}

#[cfg(test)]
mod tests {
    use crate::apk_analyzer::ApkAnalyzer;
    use std::path::PathBuf;
    use failure::Error;

    #[test]
    fn should_identify_application_name_from_apk() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let vec = analyzer.apk_summary(path);
        assert_eq!("com.example.android.multiproject", vec.m_package)
    }

    #[test]
    fn should_support_manifest_print() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/test.apk");

        let string = analyzer.manifest_print(path);
        assert_eq!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>
<manifest xmlns:android=\"http://schemas.android.com/apk/res/android\" android:versionName=\"1.0\" platformBuildVersionCode=\"23\" package=\"com.example.helloworld\" platformBuildVersionName=\"6.0-2438415\" android:versionCode=\"1\">
  <uses-sdk android:minSdkVersion=\"3\" />
  <intent-filter>
    <action android:name=\"android.intent.action.MAIN\" />
    <category android:name=\"android.intent.category.LAUNCHER\" />
  </intent-filter>
</manifest>".len(), string.len())
    }

    #[test]
    fn should_get_apk_size() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let size = analyzer.apk_file_size(path);
        assert_eq!(48196, size)
    }

    #[test]
    fn should_get_download_size() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let size = analyzer.apk_download_size(path);
        assert_eq!(39591, size)
    }

    #[test]
    fn should_cat_file() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let files = analyzer.file_cat(path, String::from("META-INF/MANIFEST.MF"));
        assert_eq!(1233, files.len());
    }

    #[test]
    fn should_list_files() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let files = analyzer.files_list(path);
        assert_eq!(18, files.len());
        // for x in files {
        //     println!("{:?}， size: {:?}, download_size: {:?}", x.path, x.raw_size, x.download_size);
        // }
    }

    #[test]
    fn should_list_dex() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let files = analyzer.dex_list(path);
        assert_eq!(1, files.len());
        assert_eq!("classes.dex", files[0].path);
    }

    #[test]
    fn should_list_dex_references() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let files = analyzer.dex_references(path);
        assert_eq!(40, files[0].referenced_method_count);
    }

    #[test]
    fn should_list_dex_package() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let node = analyzer.dex_packages(path);
        assert_eq!(10, node.class_nodes.len());
    }

    #[test]
    fn should_get_res_package() {
        let analyzer = ApkAnalyzer::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/apk/app_with_virtual_entry.apk");

        let node = analyzer.res_package(path);
        match node {
            Ok(str) => {
                println!("{:?}", str);
            },
            Err(_) => {},
        }
    }
}