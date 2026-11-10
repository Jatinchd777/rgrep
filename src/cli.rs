use clap::Parser;

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
    #[arg(short, long)]
    pub count: bool,

    /// Search directories recursively
    #[arg(short, long)]
    pub recursive: bool,

    /// Print line numbers
    #[arg(short, long)]
    pub line_number: bool,

    /// Suppress normal output
    #[arg(short, long)]
    pub quiet: bool,
}
