use std::collections::{HashMap, HashSet};
use dex::Dex;
use memmap::Mmap;
use crate::analyzer::dex::dex_package_node::DexPackageNode;

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

    pub fn construct_package_tree(&self, dexes: Vec<Dex<Mmap>>) {
        let root = DexPackageNode::new(String::from("root"), None);
        for dex in dexes {
            // package_tree(root, dex.pa)
        }
    }
}
