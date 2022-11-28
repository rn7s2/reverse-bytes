use clap::Parser;
use std::{
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    outdir: Option<String>,
    files: Vec<String>,
}

fn reverse_filename(path: &PathBuf) -> String {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .chars()
        .rev()
        .collect::<String>()
}

fn reverse_file(path: &PathBuf, outdir: &PathBuf) {
    let mut file = std::fs::File::open(path).unwrap();
    let mut outdir = outdir.clone();
    outdir.push(reverse_filename(path));
    let mut reverse_file = std::fs::File::create(outdir).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    buf.reverse();
    reverse_file.write_all(&buf).unwrap();
}

fn main() {
    let args = Args::parse();
    let outdir = match args.outdir {
        Some(dir) => {
            let dir = PathBuf::from(dir);
            assert!(dir.is_dir());
            assert!(dir.exists());
            dir
        }
        None => PathBuf::from("."),
    };

    for file in &args.files {
        let path = PathBuf::from(file);
        assert!(path.is_file());
        assert!(path.exists());
        reverse_file(&path, &outdir);
    }
}
