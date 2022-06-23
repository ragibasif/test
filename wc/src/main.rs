use clap::Parser;
mod count_total_chars;
mod count_total_line;
mod count_total_words;
mod header;
mod turn_to_vec;
use std::fs;

/// Simple program to copy .txt files
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the input file to copy text from
    #[clap(short, long, value_parser)]
    input: String,
}
/// Reads the input file
pub fn read(input: String) -> std::io::Result<()> {
    let contents = fs::read_to_string(input).expect("Problem reading the file.");
    output(contents);
    Ok(())
}
/// Writes the input file contents to the command line
pub fn output(contents: String) {
    let vec = turn_to_vec::turn_to_vec(&contents);
    header::header();
    count_total_line::count_total_lines(&vec);
    count_total_chars::count_total_chars(&vec);
    count_total_words::count_total_words(&vec);
}

fn main() {
    let args = Args::parse();

    read(args.input).expect("Error reading file.");
}
