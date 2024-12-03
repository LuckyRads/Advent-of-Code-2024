use std::io;

use crate::utils::file_reader::FileReader;

pub fn solve() -> Result<(), io::Error> {
    let input_file_path: &str = "./input/input.txt";

    let file_reader = FileReader::new(input_file_path);
    let input_lines = file_reader?.read_to_string_vec()?;
    let input_string = input_lines.concat();

    return Ok(());
}
