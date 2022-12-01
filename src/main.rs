use clap::Parser;
use std::{
    io::{Read, Write},
    path::PathBuf,
};

const BUF_SIZE: usize = 8192;

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
    let mut buf = [0u8; BUF_SIZE];

    loop {
        match file.read(&mut buf) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                } else if bytes_read < buf.len() {
                    let mut reversed_buf = Vec::from(&buf[0..bytes_read]);
                    reversed_buf.reverse();
                    reverse_file.write(&reversed_buf).unwrap();
                    break;
                } else {
                    buf.reverse();
                    reverse_file.write(&buf).unwrap();
                }
            }
            Err(_) => panic!("Error occurred when processing {:?}.", path),
        }
    }
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
