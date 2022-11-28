use clap::Parser;
use std::io::{Read, Write};

#[derive(Parser, Debug)]
struct Args {
    files: Vec<String>,
}

fn reverse_file(path: &str) {
    let mut file = std::fs::File::open(path).unwrap();
    let mut reverse_file = std::fs::File::create(path.to_owned() + ".rev").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    buf.reverse();
    reverse_file.write_all(&buf).unwrap();
}

fn main() {
    let args = Args::parse();

    for file in &args.files {
        reverse_file(file);
    }
}
