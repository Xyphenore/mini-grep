use std::convert::TryFrom;
use std::env;
use std::env::Args;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::errors::{InvalidArgumentError, InvalidSyntaxError, MiniGrepArgsError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum CaseSensitive {
    #[default]
    True,
    False,
}

impl From<bool> for CaseSensitive {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl From<CaseSensitive> for bool {
    fn from(value: CaseSensitive) -> Self {
        value == CaseSensitive::True
    }
}

#[derive(Debug)]
pub struct Command {
    pattern: String,
    filename: String,
    file: File,
    case_sensitive: bool,
}

impl Command {
    pub const IGNORE_CASE_ENV_NAME: &'static str = "IGNORE_CASE";
    const TRUE_VALUES: &'static [&'static str] = &["true", "1"];

    pub fn execute(&self) {
        let pattern = &self.pattern;
        let filename = &self.filename;
        let is_case_sensitive = if self.case_sensitive {
            "sensitive"
        } else {
            "insensitive"
        };

        let lines = self.search();

        if lines.is_empty() {
            println!(
                "The file '{filename}' does not contain any line with the case \
                {is_case_sensitive} pattern '{pattern}'.",
            )
        } else {
            println!(
                "The file '{filename}' contains these lines with the case \
                {is_case_sensitive} pattern '{pattern}':",
            );
            lines
                .into_iter()
                .for_each(|(line_no, line)| println!("{line_no}: {line}"));
        }
    }

    fn search(&self) -> Vec<(usize, String)> {
        let pattern = if self.case_sensitive {
            self.pattern.clone()
        } else {
            self.pattern.to_lowercase()
        };

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

                if self.case_sensitive {
                    line.clone()
                } else {
                    line.to_lowercase()
                }
                .contains(&pattern)
                .then_some((line_no + 1, line))
            })
            .collect()
    }

    fn build(
        pattern: String,
        filename: String,
        case_sensitive: CaseSensitive,
    ) -> Result<Command, InvalidArgumentError> {
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
                case_sensitive: bool::from(case_sensitive),
            })
    }

    fn try_from_iter(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Command, Box<dyn MiniGrepArgsError>> {
        let executable = match args.next() {
            Some(executable) => executable,
            None => panic!("Missing the executable name."),
        };

        let pattern = match args.next() {
            Some(pattern) => pattern,
            None => return Err(Box::new(InvalidSyntaxError::Missing(executable.clone()))),
        };

        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err(Box::new(InvalidSyntaxError::Missing(executable.clone()))),
        };

        if args.next().is_some() {
            return Err(Box::new(InvalidSyntaxError::TooMany(executable)));
        }

        let ignore_case_env = env::var(Self::IGNORE_CASE_ENV_NAME);
        let ignore_case = if let Ok(value) = ignore_case_env {
            Self::TRUE_VALUES.contains(&value.to_lowercase().as_str())
        } else {
            eprintln!(
                "Error during the get of the variable '{}'. The error: '{}'.",
                Self::IGNORE_CASE_ENV_NAME,
                ignore_case_env.unwrap_err(),
            );

            false
        };

        let case_sensitive = CaseSensitive::from(!ignore_case);

        Self::build(pattern, filename, case_sensitive)
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
