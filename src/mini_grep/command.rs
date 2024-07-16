use std::convert::TryFrom;
use std::env;
use std::env::Args;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::errors::{InvalidArgumentError, InvalidSyntaxError, MiniGrepArgsError};

/// Indicate that MiniGrep use a case-sensitive or not pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[doc(hidden)]
enum CaseSensitive {
    #[default]
    #[doc(hidden)]
    True,
    #[doc(hidden)]
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

/// The MiniGrep command to search each line that contains the pattern.
///
/// # Examples
///
/// ```rust
/// use std::env::args;
/// use std::process;
///
/// use crate::mini_grep::Command;
///
/// Command::try_from(args())
///     .unwrap_or_else(|error| {
///         eprintln!("{error}");
///         process::exit(error.code());
///     })
///     .execute()
/// ```
#[derive(Debug)]
pub struct Command {
    #[doc(hidden)]
    pattern: String,
    #[doc(hidden)]
    filename: String,
    #[doc(hidden)]
    file: File,
    #[doc(hidden)]
    case_sensitive: bool,
}

impl Command {
    /// The environment variable name used to activate the case-insensitive pattern.
    pub const IGNORE_CASE_ENV_NAME: &'static str = "IGNORE_CASE";

    /// All accepted values to activate the case-insensitive pattern mode.
    #[doc(hidden)]
    const TRUE_VALUES: &'static [&'static str] = &["true", "1"];

    /// Execute the MiniGrep command.
    ///
    /// Print to stdout found lines in the given filename that contains the
    /// given pattern.
    ///
    /// # Read errors
    ///
    /// Print to stderr an error message if a line cannot be read and continue the
    /// read of the file.
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

    /// Search in the file all lines containing the pattern.
    ///
    /// # Returns
    ///
    /// Returns a [`Vec`] of [`(usize, String)`] that contains all lines and its
    /// number, which contain the pattern.
    ///
    /// # Read errors
    ///
    /// Print to stderr an error message if a line cannot be read and continue the
    /// read of the file.
    #[doc(hidden)]
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

    /// Build a [`Command`].
    ///
    /// # Returns
    ///
    /// Returns a new instance of [`Command`], or an [`InvalidArgumentError`]
    /// if an error has occurred during the check of preconditions about arguments.
    ///
    /// # Panics
    ///
    /// If the absolute path cannot be converted to its string representation.
    #[doc(hidden)]
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
                        string representation fails.",
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

    /// Build a [`Command`] from an [`Iterator`] of [`String`].
    ///
    /// # Returns
    ///
    /// Returns a new instance of [`Command`], or a [`Box`] of [`MiniGrepArgsError`]
    /// if an error has occurred during the extraction of CLI arguments or during the
    /// check of preconditions about arguments.
    ///
    /// # Panics
    ///
    /// If the iterator is empty.
    #[doc(hidden)]
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

    /// Build a [`Command`] from [`Args`].
    ///
    /// # Returns
    ///
    /// Returns a new instance of [`Command`], or a [`Box`] of [`MiniGrepArgsError`]
    /// if an error has occurred during the extraction of CLI arguments or during the
    /// check of preconditions about arguments.
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
