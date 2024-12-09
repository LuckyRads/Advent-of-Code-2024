use std::{borrow::Borrow, error::Error, fmt::format, num::ParseIntError};

use crate::utils::{file_reader::FileReader, file_writer::FileWriter};

enum Direction {
    NE,
    SE,
    SW,
    NW,
}

struct XmasFinder {
    char_matrix: Vec<Vec<char>>,
}

impl XmasFinder {
    const SEARCH_WORD: &str = "MAS";

    fn new(char_matrix: Vec<Vec<char>>) -> XmasFinder {
        XmasFinder { char_matrix }
    }

    fn char_matrix(&self) -> &Vec<Vec<char>> {
        &self.char_matrix
    }

    fn search_for_word_at_pos_diag(&self, i: usize, j: usize, direction: Direction) -> bool {
        let mut found_letters = 0;
        for m in 0..Self::SEARCH_WORD.len() {
            let row_idx = match direction {
                Direction::NE | Direction::NW => i - m,
                Direction::SE | Direction::SW => i + m,
            };
            let col_idx = match direction {
                Direction::NW | Direction::SW => j - m,
                Direction::NE | Direction::SE => j + m,
            };
            if self.char_matrix[row_idx][col_idx] == Self::SEARCH_WORD.chars().nth(m).unwrap() {
                found_letters += 1;
            } else {
                return false;
            }
        }
        return found_letters == Self::SEARCH_WORD.len();
    }

    fn search_for_word_at_pos(&self, i: usize, j: usize) -> i32 {
        let positions: [(isize, isize, Direction); 4] = [
            (-1, -1, Direction::SE),
            (-1, 1, Direction::SW),
            (1, -1, Direction::NE),
            (1, 1, Direction::NW),
        ];

        let mut found_in_pos_count = 0;
        for (x_offset, y_offset, direction) in positions {
            let x = (i as isize + x_offset) as usize;
            let y = (j as isize + y_offset) as usize;
            found_in_pos_count += self.search_for_word_at_pos_diag(x, y, direction) as i32;
        }

        return (found_in_pos_count == 2) as i32;
    }
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    // let mut file_reader = FileReader::new("./input/input_test_2.txt")?;
    let mut file_reader = FileReader::new("./input/input.txt")?;
    let lines = file_reader.read_to_string_vec()?;

    let mut char_matrix: Vec<Vec<char>> = vec![];
    for i in 0..lines.len() {
        char_matrix.push(vec![]);
        for j in 0..lines[i].len() {
            char_matrix[i].push(lines[i].chars().nth(j).unwrap());
        }
    }

    let mut xmas_count = 0;
    let xmas_finder = XmasFinder::new(char_matrix);
    for i in 1..xmas_finder.char_matrix().len() - 1 {
        for j in 1..xmas_finder.char_matrix()[i].len() - 1 {
            if xmas_finder.char_matrix()[i][j] == 'A' {
                xmas_count += xmas_finder.search_for_word_at_pos(i, j);
            }
        }
    }

    let output_writer = FileWriter::new("./output/task2.txt");
    output_writer.write(&xmas_count.to_string())?;

    return Ok(());
}
