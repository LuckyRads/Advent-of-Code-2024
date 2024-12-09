use std::fs::File;

pub struct FileWriter {
    file: File,
}

impl FileWriter {
    pub fn new(file_path: &str) -> FileWriter {
        return FileWriter {
            file: File::create(file_path).unwrap(),
        };
    }

    pub fn file(&self) -> &File {
        return &self.file;
    }

    pub fn file_mut(&mut self) -> &mut File {
        return &mut self.file;
    }
}
