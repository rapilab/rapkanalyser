use crate::analyzer::dex::dex_class_node::DexClassNode;
use crate::analyzer::dex::dex_field_node::DexFieldNode;
use crate::analyzer::dex::dex_method_node::DexMethodNode;
use crate::analyzer::dex::dex_package_node::DexPackageNode;
use dex::class::Class;
use dex::field::Field;
use dex::method::Method;
use dex::string::DexString;
use dex::{Dex, Error};
use getset::{CopyGetters, Getters, MutGetters, Setters};
use memmap::Mmap;
use std::collections::{HashMap, HashSet};

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
    usages_map: ProguardUsagesMap,
}

impl PackageTreeCreator {
    pub fn new() -> PackageTreeCreator {
        PackageTreeCreator {
            proguard_map: ProguardMap::new(),
            usages_map: ProguardUsagesMap::new(),
        }
    }

    pub fn construct_package_tree(&self, dexes: Vec<Dex<Mmap>>) -> DexPackageNode {
        let mut root = DexPackageNode::new(String::from("root"), None);
        for dex_map in dexes {
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

    pub fn add_methods(&self, class_node: &mut DexClassNode, methods: Vec<&Method>) {
        for method in methods {
            let method_name = method.name().to_string();
            let return_type = method.return_type().type_descriptor().to_string();
            // method.params()
            let params = String::from("");
            let method_sig = format!("{} {}({})", method_name, return_type, params);
            let method_node = DexMethodNode::new(method_sig);
            class_node.add_method(method_node);
        }
    }
    pub fn add_fields(&self, class_node: &mut DexClassNode, fields: Vec<&Field>) {
        for field in fields {
            let field_name = field.name().to_string();
            let field_type = field.jtype().type_descriptor().to_string();
            let field_sig = format!("{} {}", field_type, field_name);

            let field_node = DexFieldNode::new(field_sig);
            class_node.add_field(field_node);
        }
    }

    pub fn package_tree(&self, root: &mut DexPackageNode, dex: Dex<Mmap>) {
        // add classes (and their methods and fields) defined in this file to the tree
        for x in dex.classes() {
            if let Ok(clz) = x {
                if let Some(source) = clz.source_file() {
                    let clz_name = source.to_string();
                    let mut typ: String = String::from("");

                    if let Ok(opt) = clz.signature() {
                        if let Some(str) = opt {
                            typ = str
                        }
                    }

                    let mut class_node = root.get_or_create_class(
                        String::from(""),
                        String::from(clz_name),
                        String::from(typ),
                    );

                    let mut methods = vec![];
                    for x in clz.methods() {
                        methods.push(x)
                    }

                    let mut fields = vec![];
                    for x in clz.fields() {
                        fields.push(x)
                    }

                    self.add_methods(&mut class_node, methods);
                    self.add_fields(&mut class_node, fields);

                    root.add_class(class_node)
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
