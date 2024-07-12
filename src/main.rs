// https://doc.rust-lang.org/book/ch12-00-an-io-project.html

#![doc(issue_tracker_base_url = "https://github.com/Xyphenore/mini-grep/issues/")]

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();

    assert!(
        args.len() >= 3,
        "Missing arguments! Please call the script like: {} pattern filename",
        args[0],
    );

    let pattern = &args[1];
    let filename = &args[2];

    assert!(
        !pattern.trim().is_empty(),
        "Cannot have a blank searched text '{pattern}'",
    );

    let file_path = Path::new(filename);
    if !file_path.is_file() {
        assert!(file_path.exists(), "The file '{filename}' does not exist");

        let absolute_path = file_path.canonicalize().unwrap_or_else(|error| {
            panic!(
                "Cannot resolve the path '{filename}' to the absolute path, \
                because this error {error}."
            );
        });

        panic!(
            "{} is not a file, it is a {}",
            absolute_path.to_str().unwrap_or_else(|| panic!(
                "Cannot convert the absolute file path to '{filename}' to its \
                string representation.",
            )),
            if absolute_path.is_dir() {
                "directory"
            } else {
                "unknown"
            },
        )
    }

    let file =
        File::open(file_path).unwrap_or_else(|_| panic!("Cannot open the file '{filename}'"));

    let buf = BufReader::new(file);

    let mut lines = buf.lines().enumerate().filter_map(|(line_no, line)| {
        let line = line.unwrap_or_else(|_| {
            eprintln!(
                "Cannot read the line {} from the file '{filename}'",
                line_no + 1,
            );

            String::default()
        });

        if line.contains(pattern) {
            Some((line_no + 1, line))
        } else {
            None
        }
    });

    if let Some((line_no, line)) = lines.next() {
        println!(
            "The file '{filename}' contains these lines with the pattern \
            '{pattern}':",
        );
        println!("{line_no}: {line}");
        lines.for_each(|(line_no, line)| println!("{line_no}: {line}"));
    } else {
        println!(
            "The file '{filename}' does not contain any line with the pattern \
            '{pattern}'.",
        )
    }
}
