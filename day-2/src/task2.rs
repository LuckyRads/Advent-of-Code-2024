use core::num;
use std::{
    fs::{canonicalize, File},
    io::{self, Write},
    vec,
};

use crate::utils::file_reader::FileReader;

fn find_abs_differences(numbers: &Vec<i32>) -> Vec<i32> {
    let mut differences = vec![];
    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        differences.push(diff.abs());
    }
    differences
}

fn has_diff_out_of_bounds(numbers: &Vec<i32>) -> bool {
    for diff in find_abs_differences(numbers) {
        if diff < 1 || diff > 3 {
            return true;
        }
    }
    false
}

fn is_vec_ascending(vector: &Vec<i32>) -> bool {
    for i in 1..vector.len() {
        if vector[i] <= vector[i - 1] {
            return false;
        }
    }
    true
}

fn is_vec_descending(vector: &Vec<i32>) -> bool {
    for i in 1..vector.len() {
        if vector[i] >= vector[i - 1] {
            return false;
        }
    }
    true
}

fn is_vec_asc_or_desc(vector: &Vec<i32>) -> bool {
    return is_vec_ascending(vector) || is_vec_descending(vector);
}

struct ProblemDampener {
    numbers: Vec<i32>,
    to_remove_index: usize,
}

impl ProblemDampener {
    fn new(numbers: Vec<i32>) -> ProblemDampener {
        ProblemDampener {
            numbers,
            to_remove_index: 0,
        }
    }

    fn next(&mut self) -> Option<Vec<i32>> {
        if self.to_remove_index >= self.numbers.len() {
            return None;
        }

        let mut numbers = self.numbers.clone();
        numbers.remove(self.to_remove_index);
        self.to_remove_index += 1;

        Option::Some(numbers)
    }
}

pub fn solve() -> Result<(), io::Error> {
    let input_file_path: &str = "./input/input.txt";

    let file_reader = FileReader::new(input_file_path);
    let input = file_reader?.read_to_string_vec()?;

    let mut safe_reports_count = 0;

    for line in input {
        let report_numbers: Vec<i32> = line
            .split_whitespace()
            .map(|str_num| str_num.parse::<i32>().unwrap())
            .collect();

        let mut safe_report = true;
        if has_diff_out_of_bounds(&report_numbers) || !is_vec_asc_or_desc(&report_numbers) {
            // For now the report is not safe
            safe_report = false;

            // Let's try the problem dampener
            let mut problem_damper = ProblemDampener::new(report_numbers);
            while let Some(dampened_numbers) = problem_damper.next() {
                if !has_diff_out_of_bounds(&dampened_numbers)
                    && is_vec_asc_or_desc(&dampened_numbers)
                {
                    safe_report = true;
                }
            }
        }

        if safe_report {
            safe_reports_count += 1;
        }
    }

    let output_file_path = "./output/task2.txt";
    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(format!("{}", safe_reports_count).as_bytes())?;

    println!(
        "Finished writing to file: {}",
        canonicalize(output_file_path)?.display()
    );

    Ok(())
}
