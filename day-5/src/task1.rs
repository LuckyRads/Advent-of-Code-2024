use std::{borrow::Borrow, error::Error, fmt::format, num::ParseIntError};

use regex::Regex;

use crate::utils::{file_reader::FileReader, file_writer::FileWriter};

struct RuleEngine {
    page_number_pairs: Vec<(String, String)>,
}

impl RuleEngine {
    fn new(input_string: &Vec<String>) -> RuleEngine {
        let rule_lines = input_string
            .iter()
            .filter(|line| line.contains("|"))
            .collect::<Vec<&String>>();
        let page_number_pairs = rule_lines
            .iter()
            .map(|line| line.trim().split("|").collect::<Vec<_>>())
            .map(|arr| (arr[0].to_string(), arr[1].to_string()))
            .collect::<Vec<(String, String)>>();

        RuleEngine { page_number_pairs }
    }

    fn get_valid_updates(&self, input_string: &Vec<String>) -> Vec<String> {
        let update_lines = input_string
            .iter()
            .filter(|line| line.contains(","))
            .collect::<Vec<&String>>();

        let rule_regexes = self
            .page_number_pairs
            .iter()
            .map(|pair| {
                (
                    Regex::new(&format!(r"{}.*{}", &pair.0, &pair.1)).unwrap(),
                    Regex::new(&format!(r"{}.*{}", &pair.1, &pair.0)).unwrap(),
                    (pair.0.to_string(), pair.1.to_string()),
                )
            })
            .collect::<Vec<(Regex, Regex, (String, String))>>();

        // for x in &update_lines {
        //     for r in &rule_regexes {
        //         println!(
        //             "{} {} {}",
        //             x,
        //             &r.0.find(x).is_some(),
        //             &r.1.find(x).is_none()
        //         )
        //     }
        // }

        update_lines
            .iter()
            .filter(|line| {
                rule_regexes
                    .iter()
                    .all(|(regex_positive, regex_negative, pair)| {
                        regex_positive.find(line).is_some()
                            && regex_negative.find(line).is_none()
                            && line.contains(&pair.0)
                            && line.contains(&pair.1)
                            || !line.contains(&pair.0)
                            || !line.contains(&pair.1)
                    })
            })
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
    }
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    // let mut file_reader = FileReader::new("./input/input_test.txt")?;
    let mut file_reader = FileReader::new("./input/input.txt")?;
    let lines = file_reader.read_to_string_vec()?;

    let rule_engine = RuleEngine::new(&lines);
    let valid_update_lines = rule_engine.get_valid_updates(&lines);

    let update_vectors = valid_update_lines
        .iter()
        .map(|line| {
            line.split(",")
                .map(|str_num| str_num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let sum: i32 = update_vectors
        .iter()
        .map(|numbers| numbers.get((numbers.len() / 2) as usize).unwrap())
        .sum();

    let output_writer = FileWriter::new("./output/task1.txt");
    output_writer.write(&sum.to_string())?;

    return Ok(());
}
