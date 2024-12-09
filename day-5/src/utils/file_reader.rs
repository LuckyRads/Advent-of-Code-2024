use std::{
    fs::File,
    io::{self, BufReader},
};

pub struct FileReader {
    buf_reader: BufReader<File>,
}

impl FileReader {
    pub fn new(file_path: &str) -> Result<Self, io::Error> {
        let file = File::open(file_path)?;
        let buf_reader = BufReader::new(file);
        Ok(FileReader { buf_reader })
    }

    pub fn buf_reader(&self) -> &BufReader<File> {
        &self.buf_reader
    }

    pub fn buf_reader_mut(&mut self) -> &mut BufReader<File> {
        &mut self.buf_reader
    }
}
