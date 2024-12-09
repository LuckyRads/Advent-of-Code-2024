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
    const SEARCH_WORD: &str = "XMAS";

    fn new(char_matrix: Vec<Vec<char>>) -> XmasFinder {
        XmasFinder { char_matrix }
    }

    fn char_matrix(&self) -> &Vec<Vec<char>> {
        &self.char_matrix
    }

    fn search_for_word_at_pos_horizontally(&self, i: usize, j: usize, is_forward: bool) -> bool {
        let mut found_letters = 0;
        for m in 0..Self::SEARCH_WORD.len() {
            let col_idx = if is_forward { j + m } else { j - m };
            if self.char_matrix[i][col_idx] == Self::SEARCH_WORD.chars().nth(m).unwrap() {
                found_letters += 1;
            } else {
                return false;
            }
        }
        return found_letters == Self::SEARCH_WORD.len();
    }

    fn search_for_word_at_pos_vertically(&self, i: usize, j: usize, is_forward: bool) -> bool {
        let mut found_letters = 0;
        for m in 0..Self::SEARCH_WORD.len() {
            let row_idx = if is_forward { i + m } else { i - m };
            if self.char_matrix[row_idx][j] == Self::SEARCH_WORD.chars().nth(m).unwrap() {
                found_letters += 1;
            } else {
                return false;
            }
        }
        return found_letters == Self::SEARCH_WORD.len();
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
        let mut xmas_count = 0;

        let has_space_to_east = j + Self::SEARCH_WORD.len() <= self.char_matrix[i].len();
        let has_space_to_west = j >= Self::SEARCH_WORD.len() - 1;
        let has_space_to_north = i >= Self::SEARCH_WORD.len() - 1;
        let has_space_to_south = i + Self::SEARCH_WORD.len() <= self.char_matrix[i].len();

        // Search for XMAS horizontally
        if has_space_to_east {
            xmas_count += if self.search_for_word_at_pos_horizontally(i, j, true) {
                // println!("{} {} east", i, j);
                1
            } else {
                0
            };
        }
        if has_space_to_west {
            xmas_count += if self.search_for_word_at_pos_horizontally(i, j, false) {
                // println!("{} {} west", i, j);
                1
            } else {
                0
            };
        }

        // Search for XMAS vertically
        if has_space_to_south {
            xmas_count += if self.search_for_word_at_pos_vertically(i, j, true) {
                // println!("{} {} south", i, j);
                1
            } else {
                0
            };
        }
        if has_space_to_north {
            xmas_count += if self.search_for_word_at_pos_vertically(i, j, false) {
                // println!("{} {} north", i, j);
                1
            } else {
                0
            };
        }

        // Search for XMAS diagonally
        // NE
        if has_space_to_north && has_space_to_east {
            xmas_count += if self.search_for_word_at_pos_diag(i, j, Direction::NE) {
                // println!("{} {} ne", i, j);
                1
            } else {
                0
            }
        }
        // SE
        if has_space_to_south && has_space_to_east {
            xmas_count += if self.search_for_word_at_pos_diag(i, j, Direction::SE) {
                // println!("{} {} se", i, j);
                1
            } else {
                0
            }
        }
        // SW
        if has_space_to_south && has_space_to_west {
            xmas_count += if self.search_for_word_at_pos_diag(i, j, Direction::SW) {
                // println!("{} {} sw", i, j);
                1
            } else {
                0
            }
        }
        // NW
        if has_space_to_north && has_space_to_west {
            xmas_count += if self.search_for_word_at_pos_diag(i, j, Direction::NW) {
                // println!("{} {} nw", i, j);
                1
            } else {
                0
            }
        }

        xmas_count
    }
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    // let mut file_reader = FileReader::new("./input/input_test.txt")?;
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
    for i in 0..xmas_finder.char_matrix().len() {
        for j in 0..xmas_finder.char_matrix()[i].len() {
            if xmas_finder.char_matrix()[i][j] == 'X' {
                xmas_count += xmas_finder.search_for_word_at_pos(i, j);
            }
        }
    }

    let output_writer = FileWriter::new("./output/task1.txt");
    output_writer.write(&xmas_count.to_string())?;

    return Ok(());
}
