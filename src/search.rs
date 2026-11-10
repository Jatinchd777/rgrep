use crate::cli::Args;
use anyhow::Result;
use std::fs;

pub fn search(args: &Args) -> Result<()> {
    println!("Searching for: {}", args.query);

    let words = &args.query;
    let file = &args.path;

    let contents = fs::read_to_string(file)?;

    for (index, line) in contents.lines().enumerate() {
        if line.contains(words) {
            println!("{}: {}", index + 1, line);
        }
    }

    Ok(())
}
