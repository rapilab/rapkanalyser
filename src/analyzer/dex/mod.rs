use crate::analyzer::dex::dex_field_node::DexFieldNode;
use crate::analyzer::dex::dex_method_node::DexMethodNode;
use crate::analyzer::dex::dex_class_node::DexClassNode;
use crate::analyzer::dex::dex_package_node::DexPackageNode;

pub mod dex_file_stats;

pub mod dex_package_node;
pub mod dex_class_node;
pub mod dex_method_node;
pub mod dex_field_node;

pub enum DexElementNode {
    DexPackage(DexPackageNode),
    DexClass(DexClassNode),
    DexMethod(DexMethodNode),
    DexField(DexFieldNode),
}