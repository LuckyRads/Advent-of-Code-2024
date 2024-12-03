use std::{error::Error, num::ParseIntError};

use regex::{CaptureMatches, Captures};

use crate::utils::{file_reader::FileReader, file_writer::FileWriter};

struct ProgramInterpreter {
    unparsed_string: String,
    number_pairs: Vec<[i32; 2]>,
}

impl ProgramInterpreter {
    pub fn new(input_string: &str) -> ProgramInterpreter {
        return ProgramInterpreter {
            unparsed_string: input_string.to_owned(),
            number_pairs: Vec::new(),
        };
    }

    pub fn parse_input(&mut self) -> Result<(), ParseIntError> {
        let instruction_regex = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

        let capture_matches = instruction_regex.captures_iter(&self.unparsed_string);

        for capture_match in capture_matches {
            let matched_strings: Vec<&str> = capture_match
                .iter()
                .skip(1)
                .map(|capture| capture.unwrap().as_str())
                .collect();

            let num_1 = matched_strings[0].parse::<i32>()?;
            let num_2 = matched_strings[1].parse::<i32>()?;

            self.number_pairs.push([num_1, num_2]);
        }
        Ok(())
    }

    pub fn calculate(&self) -> i32 {
        self.number_pairs
            .iter()
            .fold(0, |acc, pair| acc + pair[0] * pair[1])
    }
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let file_reader = FileReader::new("./input/input.txt");
    let input_lines = file_reader?.read_to_string_vec()?;
    let input_string = input_lines.concat();

    let mut program_interpreter = ProgramInterpreter::new(&input_string);
    program_interpreter.parse_input()?;

    let result = program_interpreter.calculate();
    let output_writer = FileWriter::new("./output/output.txt");
    output_writer.write(&result.to_string())?;

    return Ok(());
}
