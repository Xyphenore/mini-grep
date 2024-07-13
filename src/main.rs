// https://doc.rust-lang.org/book/ch12-00-an-io-project.html

#![doc(issue_tracker_base_url = "https://github.com/Xyphenore/mini-grep/issues/")]

use std::env::args;
use std::process;

use crate::mini_grep::Command;

mod mini_grep;

fn main() {
    let command = Command::try_from(args());
    if let Err(error) = command {
        eprintln!("{}", error);

        process::exit(error.code());
    }

    command.unwrap().execute();
}
