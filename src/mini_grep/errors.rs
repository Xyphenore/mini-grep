use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub trait MiniGrepArgsError: Debug + Display + Error {
    fn code(&self) -> i32;
}

#[derive(Debug, Clone)]
pub enum InvalidSyntaxError {
    Missing(String),
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
            "{quantity_args} arguments! Please call the script like: {executable} \
            pattern filename",
        )
    }
}

impl Error for InvalidSyntaxError {}

impl MiniGrepArgsError for InvalidSyntaxError {
    fn code(&self) -> i32 {
        match self {
            Self::Missing(_) => 126,
            Self::TooMany(_) => 127,
        }
    }
}

#[derive(Debug)]
pub enum InvalidArgumentError {
    BlankPattern(String),
    NotAFile(String, String),
    FileNotFound(String),
    CannotResolvePath(String, std::io::Error),
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
