use anyhow::Result;
use std::{env, fs};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let words = &args[1];
    let file = &args[2];

    let contents = fs::read_to_string(file)?;

    for (index, line) in contents.lines().enumerate() {
        if line.contains(words) {
            println!("{}: {}", index + 1, line);
        }
    }
    Ok(())
}
