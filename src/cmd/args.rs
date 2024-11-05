use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ccwc", version = "0.1", about = "Rust implementation of the wc command", long_about = None)]
pub struct Args {
    /// print the byte counts
    #[arg(short = 'c', long = "bytes")]
    pub bytes: bool,

    /// print the newline counts
    #[arg(short, long)]
    pub lines: bool,

    /// print the word counts
    #[arg(short, long)]
    pub words: bool,

    /// print the character counts
    #[arg(short = 'm', long = "chars")]
    pub character: bool,

    #[arg(value_name = "FILE")]
    pub file: Option<String>,
}
