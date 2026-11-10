use anyhow::Result;
use clap::Parser;
use owo_colors::OwoColorize;
use std::fs;

#[derive(Parser, Debug)]
#[command(name = "rgrep", version, about = "A fast command-line search tool")]
pub struct Args {
    /// The pattern to search for
    pub query: String,

    /// File or directory to search in
    pub path: String,

    /// Ignore case distinctions
    #[arg(short = 'i', long)]
    pub ignore_case: bool,

    /// Invert match: select non-matching lines
    #[arg(short = 'v', long)]
    pub invert_match: bool,

    /// Count matching lines instead of printing them
    #[arg(short = 'c', long)]
    pub count: bool,

    /// Search directories recursively
    #[arg(short, long)]
    pub recursive: bool,

    /// Print line numbers
    #[arg(short, long)]
    pub line_number: bool,

    /// Suppress normal output
    #[arg(short = 'q', long)]
    pub quiet: bool,
}

enum SearchMode {
    Count,
    Quiet,
    Normal,
}

fn quiet_search(args: &Args) -> Result<()> {
    print_search(args);

    let words = &args.query;
    let contents = fs::read_to_string(&args.path)?;
    let mut num = 0;

    for (index, line) in contents.lines().enumerate() {
        let num_temp = line.matches(words).count();

        if line.contains(words) {
            num += num_temp;
        }
    }

    println!("{}", num);

    Ok(())
}

fn colorize(line: &str, words: &str) -> String {
    line.replace(words, &format!("{}", words.truecolor(120, 220, 150).bold()))
}

fn print_search(args: &Args) {
    println!("\nfile : {}\n", args.path);
}

fn count_lines_search(args: &Args) -> Result<()> {
    print_search(args);
    let words = &args.query;
    let file = &args.path;
    let contents = fs::read_to_string(file)?;

    for (index, line) in contents.lines().enumerate() {
        if line.contains(words) {
            println!("match: {}", index + 1);
        }
    }

    Ok(())
}

fn search(args: &Args) -> Result<()> {
    print_search(args);

    let words = &args.query;
    let file = &args.path;

    let contents = fs::read_to_string(file)?;

    for (index, line) in contents.lines().enumerate() {
        if line.contains(words) {
            println!("{}: {}", index + 1, colorize(line, words));
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mode = if args.count {
        SearchMode::Count
    } else if args.quiet {
        SearchMode::Quiet
    } else {
        SearchMode::Normal
    };

    match mode {
        SearchMode::Normal => search(&args)?,
        SearchMode::Count => count_lines_search(&args)?,
        SearchMode::Quiet => quiet_search(&args)?,
    }

    Ok(())
}
