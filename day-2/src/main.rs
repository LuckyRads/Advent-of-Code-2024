mod utils;

use std::{
    fs::{canonicalize, File},
    io::{self, Write},
    vec,
};

use utils::file_reader::FileReader;

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

fn main() -> Result<(), io::Error> {
    let input_file_path: &str = "./input/input.txt";

    let file_reader = FileReader::new(input_file_path);
    let input = file_reader?.read_to_string_vec()?;

    let mut safe_reports_count = 0;

    for line in input {
        let report_numbers: Vec<i32> = line
            .split_whitespace()
            .map(|str_num| str_num.parse::<i32>().unwrap())
            .collect();

        if has_diff_out_of_bounds(&report_numbers) || !is_vec_asc_or_desc(&report_numbers) {
            continue;
        }

        safe_reports_count += 1;
    }

    let output_file_path = "./output/task1.txt";
    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(format!("{}", safe_reports_count).as_bytes())?;

    println!(
        "Finished writing to file: {}",
        canonicalize(output_file_path)?.display()
    );

    return Ok(());
}
