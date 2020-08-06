use std::collections::{HashMap, HashSet};
use dex::{Dex, Error};
use memmap::Mmap;
use crate::analyzer::dex::dex_package_node::DexPackageNode;
use dex::class::Class;
use dex::string::DexString;


#[derive(Debug, Clone)]
pub struct ProguardUsagesMap {
    classes: HashSet<String>,
    methods_by_class: HashMap<String, String>,
    fields_by_class: HashMap<String, String>,
}

impl ProguardUsagesMap {
    pub fn new() -> ProguardUsagesMap {
        ProguardUsagesMap {
            classes: Default::default(),
            methods_by_class: Default::default(),
            fields_by_class: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProguardMap {}

impl ProguardMap {
    pub fn new() -> ProguardMap {
        ProguardMap {}
    }
}

#[derive(Debug, Clone)]
pub struct PackageTreeCreator {
    proguard_map: ProguardMap,
    usages_map: ProguardUsagesMap
}

impl PackageTreeCreator {
    pub fn new() -> PackageTreeCreator {
        PackageTreeCreator {
            proguard_map: ProguardMap::new(),
            usages_map: ProguardUsagesMap::new()
        }
    }

    pub fn construct_package_tree(&self, dexes: Vec<Dex<Mmap>>) -> DexPackageNode {
        let mut root = DexPackageNode::new(String::from("root"), None);
        for dex_map in dexes {
            // let dex = DexFile::from(dex_map);
            self.package_tree(&mut root, dex_map)
        }

        root
    }

    // pub fn get_all_method(&self, dex: &DexFile) {}
    // pub fn get_all_field(&self, dex: &DexFile) {}
    // pub fn get_all_type(&self, dex: &DexFile) {}

    pub fn add_lost_method(&self) {}
    pub fn add_lost_field(&self) {}
    pub fn add_proguard_removed_def(&self) {}

    pub fn package_tree(&self, root: &mut DexPackageNode, dex: Dex<Mmap>) {
        // add classes (and their methods and fields) defined in this file to the tree
        for x in dex.classes() {
            if let Ok(clz) = x {
                if let Some(source) = clz.source_file() {
                    let clz_name = source.to_string();
                    let mut typ: String = String::from("");

                    if let Ok(opt) = clz.signature() {
                        if let Some(str) = opt { typ = str }
                    }

                    root.get_or_create_class(String::from(""), String::from(clz_name), String::from(typ))
                }

            }
        }

        // add method references which are not in a class defined in this dex file to the tree
        self.add_lost_method();

        // add field references which are not in a class defined in this dex file
        self.add_lost_field();

        // add classes, methods and fields removed by Proguard
        self.add_proguard_removed_def();
    }
}
