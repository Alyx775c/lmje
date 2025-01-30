use std::path::PathBuf;

pub struct FileHandler {
    pub file: Option<PathBuf>,
    pub skill_folder: Option<PathBuf>,
    //pub personality_folder: Option<PathBuf>
}

impl FileHandler {
    pub const fn new() -> Self {
        Self { file: None, skill_folder: None }
    }

    pub fn set_folder(&mut self, file: PathBuf) {
        self.file = Some(file);
    }

    pub fn get_folder(&self) -> Option<PathBuf> {
        self.file.clone()
    }

    pub fn set_skill_folder(&mut self, file: PathBuf) {
        self.skill_folder = Some(file);
    }
}