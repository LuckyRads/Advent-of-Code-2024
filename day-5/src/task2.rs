use std::{borrow::Borrow, error::Error, fmt::format, num::ParseIntError};

use crate::utils::{file_reader::FileReader, file_writer::FileWriter};

pub fn solve() -> Result<(), Box<dyn Error>> {
    // let mut file_reader = FileReader::new("./input/input_test_2.txt")?;
    let mut file_reader = FileReader::new("./input/input.txt")?;
    let lines = file_reader.read_to_string_vec()?;

    let output_writer = FileWriter::new("./output/task2.txt");
    // output_writer.write(&xmas_count.to_string())?;

    return Ok(());
}
