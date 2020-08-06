use crate::analyzer::dex::dex_method_node::DexMethodNode;
use crate::analyzer::dex::DexElementNode;
use crate::analyzer::dex::dex_field_node::DexFieldNode;

#[derive(Clone)]
pub struct DexClassNode {
    name: String,
    child: Vec<DexElementNode>
}

impl DexClassNode {
    pub fn new(name: String) -> DexClassNode {
        DexClassNode {
            name,
            child: vec![]
        }
    }

    pub fn add_method(&mut self, method_node: DexMethodNode) {
        self.child.push(DexElementNode::DexMethod(method_node));
    }
    pub fn add_field(&mut self, field_node: DexFieldNode) {
        self.child.push(DexElementNode::DexField(field_node));
    }
}
