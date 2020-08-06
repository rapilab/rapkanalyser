#[derive(Clone)]
pub struct DexFieldNode {
    pub name: String
}

impl DexFieldNode {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}
