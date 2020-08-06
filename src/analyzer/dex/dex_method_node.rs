pub struct DexMethodNode {
    pub method_sig: String
}

impl DexMethodNode {
    pub fn new(method_sig: String) -> DexMethodNode {
        DexMethodNode {
            method_sig
        }
    }
}