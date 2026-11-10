use anyhow::Result;
use clap::Parser;
use owo_colors::OwoColorize;
use std::fs;
use std::path::Path;

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
    #[arg(short = 'r', long)]
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

fn recursive_search(path: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            files.extend(recursive_search(&path)?);
        } else if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}

fn quiet_search(path: &Path, contents: &str, args: &Args) -> Result<()> {
    print_search(path);

    let words = &args.query;
    let mut num = 0;

    for line in contents.lines() {
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

fn print_search(path: &Path) {
    println!(
        "\nfile : {}\n",
        path.display().truecolor(100, 190, 255).bold()
    );
}

fn count_lines_search(path: &Path, contents: &str, args: &Args) -> Result<()> {
    print_search(path);
    let words = &args.query;

    for (index, line) in contents.lines().enumerate() {
        if line.contains(words) {
            println!("match: {}", index + 1);
        }
    }

    Ok(())
}

fn search_file(path: &Path, contents: &str, args: &Args) -> Result<()> {
    print_search(path);

    for (index, line) in contents.lines().enumerate() {
        if line.contains(&args.query) {
            println!("{}: {}", index + 1, colorize(line, &args.query));
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

    let files = if args.recursive {
        recursive_search(Path::new(&args.path))?
    } else {
        vec![Path::new(&args.path).to_path_buf()]
    };

    for file in &files {
        let contents = fs::read(file)?;

        let Ok(contents) = std::str::from_utf8(&contents) else {
            continue;
        };

        if !contents.contains(&args.query) {
            continue;
        }

        match mode {
            SearchMode::Normal => search_file(file, &contents, &args)?,
            SearchMode::Count => count_lines_search(file, &contents, &args)?,
            SearchMode::Quiet => quiet_search(file, &contents, &args)?,
        };
    }

    Ok(())
}
