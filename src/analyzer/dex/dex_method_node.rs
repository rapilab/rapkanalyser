#[derive(Clone)]
pub struct DexMethodNode {
    pub name: String,
}

impl DexMethodNode {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
