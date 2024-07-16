// https://doc.rust-lang.org/book/ch12-00-an-io-project.html

#![doc(issue_tracker_base_url = "https://github.com/Xyphenore/mini-grep/issues/")]

use std::env::args;
use std::process;

use crate::mini_grep::Command;

mod mini_grep;

/// Executable script to start mini-grep.
///
/// Get args given on CLI and get the environment variable 'IGNORE_CASE'.
///
/// # Panics
///
/// - If any method ([`Command::try_from()`] or [`Command::execute()`]) panics.
fn main() {
    Command::try_from(args())
        .unwrap_or_else(|error| {
            eprintln!("{error}");
            process::exit(error.code());
        })
        .execute()
}
