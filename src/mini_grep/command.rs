use std::convert::TryFrom;
use std::env::Args;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::errors::{InvalidArgumentError, InvalidSyntaxError, MiniGrepArgsError};

#[derive(Debug)]
pub struct Command {
    pattern: String,
    filename: String,
    file: File,
}

impl Command {
    pub fn execute(&self) {
        let pattern = &self.pattern;
        let filename = &self.filename;

        let lines = self.search();

        if lines.is_empty() {
            println!(
                "The file '{filename}' does not contain any line with the pattern \
                '{pattern}'.",
            )
        } else {
            println!("The file '{filename}' contains these lines with the pattern '{pattern}':",);
            lines
                .into_iter()
                .for_each(|(line_no, line)| println!("{line_no}: {line}"));
        }
    }

    fn search(&self) -> Vec<(usize, String)> {
        let pattern = &self.pattern;
        let filename = &self.filename;

        BufReader::new(&self.file)
            .lines()
            .enumerate()
            .filter_map(|(line_no, line)| {
                let line = line.unwrap_or_else(|error| {
                    eprintln!(
                        "Cannot read the line {} from the file '{filename}', due \
                             to this error {error}.",
                        line_no + 1,
                    );

                    String::default()
                });

                line.contains(pattern).then_some((line_no + 1, line))
            })
            .collect()
    }

    fn build(pattern: String, filename: String) -> Result<Command, InvalidArgumentError> {
        if pattern.trim().is_empty() {
            return Err(InvalidArgumentError::BlankPattern(pattern));
        }

        let file_path = Path::new(&filename);

        if !file_path.is_file() {
            if !file_path.exists() {
                return Err(InvalidArgumentError::FileNotFound(filename));
            }

            let absolute_path = file_path.canonicalize().map_err(|error| {
                InvalidArgumentError::CannotResolvePath(filename.to_owned(), error)
            })?;

            return Err(absolute_path
                .to_str()
                .map(|file_path| {
                    let file_type = if absolute_path.is_dir() {
                        "directory"
                    } else {
                        "unknown"
                    };

                    InvalidArgumentError::NotAFile(file_path.to_owned(), file_type.to_owned())
                })
                .unwrap_or_else(|| {
                    panic!(
                        "The absolute path conversion ('{filename}') to its \
                        string representation fails."
                    )
                }));
        };

        File::open(file_path)
            .map_err(|error| InvalidArgumentError::NotAReadableFile(filename.to_owned(), error))
            .map(|file| Self {
                pattern,
                file,
                filename,
            })
    }

    fn try_from_iter(
        args: impl Iterator<Item = String>,
    ) -> Result<Command, Box<dyn MiniGrepArgsError>> {
        let args: Vec<_> = args.collect();

        let amount_args = args.len();

        if amount_args != 3 {
            if amount_args == 0 {
                panic!("Missing the executable name.");
            }

            let error_ctor = if amount_args > 3 {
                InvalidSyntaxError::TooMany
            } else {
                InvalidSyntaxError::Missing
            };

            return Err(Box::new(error_ctor(args[0].to_owned())));
        }

        Self::build(args[1].to_owned(), args[2].to_owned())
            .map_err(|error| Box::new(error) as Box<dyn MiniGrepArgsError>)
    }
}

impl TryFrom<Args> for Command {
    type Error = Box<dyn MiniGrepArgsError>;

    fn try_from(value: Args) -> Result<Command, Self::Error> {
        Self::try_from_iter(value)
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MiniGrep command searching the pattern '{}' in the file '{}'.",
            &self.pattern, &self.filename,
        )
    }
}
