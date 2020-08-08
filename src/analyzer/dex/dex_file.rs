use dex::class::{Class, ClassDefItem};
use dex::Dex;
use memmap::Mmap;

#[derive(Debug)]
pub struct DexFile {
    pub(crate) classes: Vec<Class>,
    class_defs: Vec<ClassDefItem>,
}

impl DexFile {
    pub fn from(dex: Dex<Mmap>) -> DexFile {
        let mut file = DexFile {
            classes: vec![],
            class_defs: vec![],
        };

        for x in dex.class_defs() {
            match x {
                Ok(def) => {
                    file.class_defs.push(def);
                }
                Err(_) => {}
            }
        }
        for x in dex.classes() {
            match x {
                Ok(clz) => {
                    file.classes.push(clz);
                }
                Err(_) => {}
            }
        }

        file
    }
}
