mod task1;
mod task2;
mod utils;

use std::io;

fn main() -> Result<(), io::Error> {
    // task1::solve()?;
    task2::solve()?;
    return Ok(());
}
