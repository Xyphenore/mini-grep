// https://doc.rust-lang.org/book/ch12-00-an-io-project.html

#![doc(issue_tracker_base_url = "https://github.com/Xyphenore/mini-grep/issues/")]

use std::env::args;
use std::process;

use crate::mini_grep::Command;

mod mini_grep;

fn main() {
    Command::try_from(args())
        .unwrap_or_else(|error| {
            eprintln!("{error}");
            process::exit(error.code());
        })
        .execute()
}
