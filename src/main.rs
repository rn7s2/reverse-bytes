use clap::Parser;
use progress_bar::*;
use std::{
    io::{Read, Write},
    path::PathBuf, fs::remove_file,
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
    let mut reverse_file = std::fs::File::create(&outdir).unwrap();
    let mut buf = [0u8; BUF_SIZE];
    let mut cnt = 0;
    let total = file.metadata().unwrap().len() as usize;
    let mut last_percent = 0;

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
                cnt += bytes_read;
                let now_percent = (cnt as f64 * 100.0 / total as f64).ceil() as usize;
                if now_percent != last_percent {
                    set_progress_bar_progression(now_percent);
                    last_percent = now_percent;
                }
            }
            Err(_) => {
                print_progress_bar_info(
                    "Error",
                    &format!("Error occurred when processing {}.", path.display()),
                    Color::Red,
                    Style::Bold,
                );
                remove_file(&outdir).unwrap();
                return;
            }
        }
    }

    print_progress_bar_info(
        "Success",
        &format!("Finished reversing {}.", path.display()),
        Color::Green,
        Style::Bold,
    );
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
        init_progress_bar(100);
        set_progress_bar_action("Reversing", Color::Blue, Style::Bold);
        if !path.is_file() {
            print_progress_bar_info(
                "Failed",
                &format!("{} is not a file.", file),
                Color::Red,
                Style::Bold,
            );
            continue;
        }
        if !path.exists() {
            print_progress_bar_info(
                "Failed",
                &format!("{} does not exists.", file),
                Color::Red,
                Style::Bold,
            );
            continue;
        }
        reverse_file(&path, &outdir);
        finalize_progress_bar();
    }
}
