use std::path::PathBuf;

pub trait Archive {
    fn get_path(&self) -> PathBuf;
}
