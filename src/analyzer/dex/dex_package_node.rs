#[derive(Clone)]
pub struct DexPackageNode {
    name: String,
    package_name: Option<String>,
}

impl DexPackageNode {
    pub fn new(name: String, package_name: Option<String>) -> DexPackageNode {
        DexPackageNode {
            name,
            package_name,
        }
    }

    pub fn combine(parent_package: String, child_name: String) -> String {
        if parent_package == "" {
            child_name.clone() + "." + &*child_name
        } else {
            parent_package + "." + &*child_name
        }
    }

    pub fn get_or_create_class(&self, parent_package: String, class_name: String, typ: String) {
        let option = class_name.find(".");
        match option {
            None => {

            },
            Some(size) => {

            },
        }
    }
}
