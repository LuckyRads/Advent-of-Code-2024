use crate::utils::file_writer::FileWriter;

use std::io::{self, Write};

impl FileWriter {
    pub fn write(&self, content: &str) -> Result<(), io::Error> {
        self.file().write_all(format!("{}\n", content).as_bytes())
    }
}
