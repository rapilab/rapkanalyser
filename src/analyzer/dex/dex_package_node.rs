#[derive(Clone)]
pub struct DexPackageNode {
    name: String,
    package_name: Option<String>
}

impl DexPackageNode {
    pub fn new(name: String, package_name: Option<String>) -> DexPackageNode {
        DexPackageNode {
            name,
            package_name
        }
    }
}
