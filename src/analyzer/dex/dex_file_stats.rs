use dex::Dex;
use memmap::Mmap;

pub struct DexFileStats {
    pub class_count: usize,
    pub defined_method_count: usize,
    pub referenced_method_count: usize,
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
                    for _method in clz.methods() {
                        defined_method_count = defined_method_count + 1;
                    }
                }
                Err(_) => {}
            }
        }

        for _x in dex.method_ids() {
            referenced_method_count = referenced_method_count + 1;
        }

        DexFileStats {
            class_count,
            defined_method_count,
            referenced_method_count,
        }
    }
}
