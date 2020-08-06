use crate::analyzer::dex::dex_class_node::DexClassNode;

#[derive(Clone)]
pub struct DexPackageNode {
    pub(crate) name: String,
    package_name: Option<String>,
    pub(crate) class_nodes: Vec<DexClassNode>
}

impl DexPackageNode {
    pub fn new(name: String, package_name: Option<String>) -> DexPackageNode {
        DexPackageNode {
            name,
            package_name,
            class_nodes: vec![]
        }
    }

    pub fn combine(parent_package: String, child_name: String) -> String {
        if parent_package == "" {
            child_name.clone() + "." + &*child_name
        } else {
            parent_package + "." + &*child_name
        }
    }

    pub fn add_class(&mut self, class_node: DexClassNode) {
        self.class_nodes.push((class_node));
    }

    pub fn get_or_create_class(&self, parent_package: String, class_name: String, typ: String) -> DexClassNode {
        let mut dex_class: DexClassNode = DexClassNode::new(String::from(class_name.clone()));
        let option = class_name.find(".");
        match option {
            None => {}
            Some(size) => {

            }
        };

        dex_class
    }
}

#[cfg(test)]
mod tests {
    use crate::analyzer::dex::dex_package_node::DexPackageNode;

    #[test]
    fn should_create_class() {
        let node = DexPackageNode::new(String::from("Hello"), Some(String::from("com.phodal")));
        node.get_or_create_class(
            String::from("com.phodal"),
            String::from("Hello"),
            String::from("AA"));

    }
}
