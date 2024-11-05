mod cmd;

use std::{
    fs::File,
    io::{self, Read},
};

use clap::Parser;
use cmd::args::Args;

fn main() {
    let args = Args::parse();

    if let Some(ref file_path) = args.file {
        let mut f = File::open(file_path).unwrap();
        let mut content = String::new();
        f.read_to_string(&mut content).unwrap();
        process(&args, &content, Some(file_path));
    } else {
        println!("No file specified. Reading from standard input.");
        let mut stdin = io::stdin();
        let mut content = String::new();
        stdin.read_to_string(&mut content).unwrap();
        process(&args, &content, None);
    }
}

fn process(args: &Args, content: &str, file_path: Option<&str>) {
    let mut results = Vec::new();
    if args.bytes {
        results.push(count_bytes(content));
    }

    if args.lines {
        results.push(count_lines(content));
    }

    if args.words {
        results.push(count_words(content));
    }

    if args.character {
        results.push(count_chars(content))
    }

    if results.is_empty() {
        results.push(count_bytes(content));
        results.push(count_lines(content));
        results.push(count_words(content));
    }

    let mut output = String::new();
    for res in results {
        output.push_str(&res.to_string());
        output.push(' ');
    }
    if let Some(file_path) = file_path {
        output.push_str(file_path);
    }

    println!("{}", output);
}

fn count_bytes(buf: &str) -> usize {
    buf.as_bytes().len()
}

fn count_lines(buf: &str) -> usize {
    buf.lines().count()
}

fn count_words(buf: &str) -> usize {
    let words = buf.split_whitespace();
    words.count()
}

fn count_chars(buf: &str) -> usize {
    buf.chars().count()
}
