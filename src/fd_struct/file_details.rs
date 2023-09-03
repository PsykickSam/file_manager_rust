#[derive(Debug, Clone)]
pub struct FileDetails {
    pub index: u16,
    pub name: String,
    pub path: String,
    pub is_file: bool,
    pub is_dir: bool,
    pub is_back: bool,
}

impl FileDetails {
    pub fn new(index: u16, name: String, path: String, is_file: bool, is_dir: bool) -> FileDetails {
        FileDetails { index, name, path, is_file, is_dir, is_back: false }
    }

    pub fn newb(index: u16, name: String, path: String, is_file: bool, is_dir: bool, is_back: bool) -> FileDetails {
        FileDetails { index, name, path, is_file, is_dir, is_back }
    }
}