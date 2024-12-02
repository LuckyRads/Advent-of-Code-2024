mod utils;

use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
};

use utils::file_reader::FileReader;

// struct FileReader {
//     buf_reader: BufReader<File>,
// }

// impl FileReader {
//     pub fn new(file_path: &str) -> Result<Self, io::Error> {
//         let file = File::open(file_path)?;
//         let buf_reader = BufReader::new(file);
//         Ok(FileReader { buf_reader })
//     }

//     pub fn read_to_string_vec(self) -> Result<Vec<String>, std::io::Error> {
//         let result: Vec<String> = self.buf_reader.lines().map(|line| line.unwrap()).collect();
//         Ok(result)
//     }
// }

fn main() -> Result<(), io::Error> {
    let input_file_path: &str = "./input/input.txt";

    let file_reader = FileReader::new(input_file_path);
    let input = file_reader?.read_to_string_vec()?;

    for line in input {
        let report_numbers: Vec<i32> = line
            .split_whitespace()
            .map(|str_num| str_num.parse::<i32>().unwrap())
            .collect();
    }
    return Ok(());
}
