use std::path::PathBuf;

pub struct FileHandler {
    file: Option<PathBuf>
}

impl FileHandler {
    pub const fn new() -> Self {
        Self { file: None }
    }

    pub fn set_folder(&mut self, file: PathBuf) {
        self.file = Some(file);
    }

    pub fn get_folder(&self) -> Option<PathBuf> {
        self.file.clone()
    }
}