use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
struct Args {
    /// The path of the directory to print
    path: Option<String>,

    /// Print hidden files
    #[arg(short, long)]
    all: bool,

    /// Recursive depth
    #[arg(short, long, default_value = "1")]
    depth: usize,

    /// The maximum number of files to print under a directory
    #[arg(long, default_value = "3")]
    directory_length: usize,

    /// Output the result as JSON
    #[arg(short, long)]
    json: bool,

    /// Tab size
    #[arg(short, long, default_value = "4")]
    tab_size: usize,

    /// The style of the output
    #[arg(short, long, default_value = "ascii")]
    style: Style,
}

#[derive(Copy, Clone, ValueEnum, Debug)]
enum Style {
    Ascii,
    Emoji,
}

fn main() {
    let args = Args::parse();
}
