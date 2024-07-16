use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Indicate that [`Command::try_from()`](crate::Command::try_from) or
/// [`Command::build`](crate::Command::build) receives a bad argument from CLI.
pub trait MiniGrepArgsError: Debug + Display + Error {
    /// Get the code used to stop the run of MiniGrep.
    fn code(&self) -> i32;
}

/// Indicate that [`Command::try_from()`](crate::Command::try_from) receives not
/// enough or too many arguments from CLI.
#[derive(Debug, Clone)]
pub enum InvalidSyntaxError {
    /// Indicate that [`Command::try_from()`](crate::Command::try_from) receives not
    /// enough arguments from CLI.
    Missing(String),
    /// Indicate that [`Command::try_from()`](crate::Command::try_from) receives too
    /// many arguments from CLI.
    TooMany(String),
}

impl Display for InvalidSyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (quantity_args, executable) = match self {
            Self::Missing(executable) => ("Missing", executable),
            Self::TooMany(executable) => ("Too many", executable),
        };

        write!(
            f,
            "{quantity_args} arguments. Call the script like: {executable} \
            pattern filename",
        )
    }
}

impl Error for InvalidSyntaxError {}

impl MiniGrepArgsError for InvalidSyntaxError {
    /// Get the code used to stop the run of MiniGrep.
    ///
    /// # Returns
    ///
    /// - 126: If not enough arguments are given to CLI.
    /// - 127: If too many arguments are given to CLI.
    fn code(&self) -> i32 {
        match self {
            Self::Missing(_) => 126,
            Self::TooMany(_) => 127,
        }
    }
}

/// Indicate that [`Command::build()`](crate::Command::build) receives a bad argument
/// from CLI.
#[derive(Debug)]
pub enum InvalidArgumentError {
    /// Indicate that [`Command::build()`](crate::Command::build) receives a blank
    /// pattern, from CLI.
    BlankPattern(String),
    /// Indicate that [`Command::build()`](crate::Command::build) receives a path to
    /// a not file (directory or anything else), from CLI.
    NotAFile(String, String),
    /// Indicate that [`Command::build()`](crate::Command::build) receives a path
    /// pointing to a not existing file, from CLI.
    FileNotFound(String),
    /// Indicate that [`Command::build()`](crate::Command::build) receives a
    /// relative path that cannot be converted to the absolute path, from CLI.
    CannotResolvePath(String, std::io::Error),
    /// Indicate that [`Command::build()`](crate::Command::build) receives a path
    /// pointing to a not readable file, from CLI.
    NotAReadableFile(String, std::io::Error),
}

impl Display for InvalidArgumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::BlankPattern(pattern) => {
                format!("Cannot have a blank searched text '{pattern}'.")
            }
            Self::NotAFile(filename, file_type) => {
                format!("'{filename}' is not a file, it is a {file_type}.")
            }
            Self::FileNotFound(filename) => format!("The file '{filename}' does not exist."),
            Self::CannotResolvePath(filename, error) => format!(
                "Cannot resolve the path ('{filename}') to the absolute path, \
                due to this error {error}."
            ),
            Self::NotAReadableFile(filename, error) => {
                format!("Cannot open the file '{filename}', due to this error {error}.")
            }
        };

        write!(f, "{}", msg)
    }
}

impl Error for InvalidArgumentError {}

impl MiniGrepArgsError for InvalidArgumentError {
    /// Get the code used to stop the run of MiniGrep.
    ///
    /// # Returns
    ///
    /// - 130: If receives a blank pattern.
    /// - 131: If receives a path pointing to a directory or anything else than a file.
    /// - 132: If receives a path pointing to a not existing file.
    /// - 133: If receives a relative path that cannot be resolved to an absolute path.
    /// - 134: If receives a path to a not readable file.
    fn code(&self) -> i32 {
        match self {
            Self::BlankPattern(_) => 130,
            Self::NotAFile(..) => 131,
            Self::FileNotFound(_) => 132,
            Self::CannotResolvePath(..) => 133,
            Self::NotAReadableFile(..) => 134,
        }
    }
}
