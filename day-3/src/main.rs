mod task1;
mod task2;
mod utils;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <task_number>");
        eprintln!("\nProvided arguments: {:?}", args);
        std::process::exit(1);
    }

    if args[1] == "1" {
        task1::solve()?;
    } else if args[1] == "2" {
        task2::solve()?;
    } else {
        eprintln!("Invalid task number");
        std::process::exit(1);
    }
    println!("Task completed successfully");
    return Ok(());
}
