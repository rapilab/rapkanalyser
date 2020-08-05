use memmap::Mmap;
use dex::{Dex, Error};
use dex::class::Class;

pub struct DexFileStats {
    pub class_count: usize,
    pub defined_method_count: usize,
    pub referenced_method_count: usize
}

impl DexFileStats {
    pub fn create(dex: Dex<Mmap>) -> DexFileStats {
        let class_count = dex.classes().len();
        let mut defined_method_count = 0;
        let mut referenced_method_count = 0;
        for clz_result in dex.classes() {
            match clz_result {
                Ok(clz) => {
                    defined_method_count += clz.methods().len();
                },
                Err(_) => {},
            }
        }

        DexFileStats {
            class_count,
            defined_method_count,
            referenced_method_count,
        }
    }
}
