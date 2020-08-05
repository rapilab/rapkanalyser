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
        let mut class_count = 0;
        let mut defined_method_count = 0;
        let mut referenced_method_count = 0;
        for clz_result in dex.classes() {
            class_count = class_count + 1;
            match clz_result {
                Ok(clz) => {
                    for method in clz.methods() {
                        defined_method_count = defined_method_count + 1;
                    }
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
