use crate::utils::file_reader::FileReader;
use std::io::BufRead;

impl FileReader {
    pub fn read_to_string_vec(&mut self) -> Result<Vec<String>, std::io::Error> {
        let result: Vec<String> = self
            .buf_reader_mut()
            .lines()
            .map(|line| line.unwrap())
            .collect();
        Ok(result)
    }
}
