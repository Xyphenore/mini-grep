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

        let mut lines =
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
                });

        if let Some((line_no, line)) = lines.next() {
            println!("The file '{filename}' contains these lines with the pattern '{pattern}':",);
            println!("{line_no}: {line}");
            lines.for_each(|(line_no, line)| println!("{line_no}: {line}"));
        } else {
            println!(
                "The file '{filename}' does not contain any line with the pattern \
                '{pattern}'.",
            )
        }
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

            return Err(absolute_path.to_str().map_or(
                InvalidArgumentError::CannotConvertPathToString(filename),
                |file_path| {
                    let file_type = if absolute_path.is_dir() {
                        "directory"
                    } else {
                        "unknown"
                    };

                    InvalidArgumentError::NotAFile(file_path.to_owned(), file_type.to_owned())
                },
            ));
        };

        File::open(file_path)
            .map_err(|error| InvalidArgumentError::NotAReadableFile(filename.to_owned(), error))
            .map(|file| Self {
                pattern,
                file,
                filename,
            })
    }
}

impl TryFrom<Args> for Command {
    type Error = Box<dyn MiniGrepArgsError>;

    fn try_from(value: Args) -> Result<Command, Self::Error> {
        let args: Vec<_> = value.collect();

        if args.len() != 3 {
            let error_ctor = if args.len() > 3 {
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

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MiniGrep command searching the pattern '{}' in the file '{}'.",
            &self.pattern, &self.filename,
        )
    }
}
