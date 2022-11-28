use clap::Parser;
use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
struct Args {
    files: Vec<String>,
}

fn reverse_filename(path: &str) -> String {
    let path = Path::new(path);
    let mut result: PathBuf = path.to_path_buf();
    result.set_file_name(
        path.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .chars()
            .rev()
            .collect::<String>(),
    );
    result.to_str().unwrap().to_string()
}

fn reverse_file(path: &str) {
    let mut file = std::fs::File::open(path).unwrap();
    let mut reverse_file = std::fs::File::create(reverse_filename(path)).unwrap();
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
