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

#[cfg(test)]
mod tests {
    use super::*;

    const RESOURCES_DIR: &str = "resources";
    const EXAMPLE_FILE: &str = "resources/example.txt";

    #[cfg(target_os = "windows")]
    const NOT_READABLE_FILE: &str = "resources/not_readable_file.txt";
    #[cfg(target_os = "unix")]
    const NOT_READABLE_FILE: &str = "resources/not_readable_unix_file";

    #[derive(Debug, Clone, Copy)]
    pub enum PatternType {
        Blank,
        Empty,
    }

    impl Display for PatternType {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let pattern_type = match self {
                Self::Blank => "blank",
                Self::Empty => "empty",
            };

            write!(f, "{pattern_type}")
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum PathType {
        Directory,
        #[cfg(target_os = "unix")]
        Unknown,
    }

    impl PathType {
        fn build_part(&self) -> &str {
            match self {
                Self::Directory => "a directory",
                #[cfg(target_os = "unix")]
                Self::Unknown => "an unknown node type",
            }
        }
    }

    impl Display for PathType {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let path_type = match self {
                Self::Directory => "directory",
                #[cfg(target_os = "unix")]
                Self::Unknown => "unknown",
            };

            write!(f, "{path_type}")
        }
    }

    mod build {
        use super::*;

        #[test]
        fn with_all_args() {
            let res = Command::build("pattern".to_owned(), EXAMPLE_FILE.to_owned());

            assert!(
                res.is_ok(),
                "Command should be built with a valid pattern and a valid filename. \
                The error: '{}'.",
                res.unwrap_err(),
            )
        }

        mod with_invalid_pattern {
            use std::any::Any;

            use super::*;

            #[test]
            fn as_a_blank_pattern() {
                let res = Command::build(" ".to_owned(), EXAMPLE_FILE.to_owned());
                check_result(res, PatternType::Blank);
            }

            #[test]
            fn as_an_empty_pattern() {
                let res = Command::build("".to_owned(), EXAMPLE_FILE.to_owned());
                check_result(res, PatternType::Empty);
            }

            fn check_result(res: Result<Command, InvalidArgumentError>, pattern_type: PatternType) {
                assert!(
                    res.is_err(),
                    "The command should not be built with a {pattern_type} pattern.",
                );

                let error = res.unwrap_err();

                match error {
                    InvalidArgumentError::BlankPattern(_) => {}
                    _ => {
                        panic!(
                            "Invalid sub error {:?}, must be an \
                            InvalidArgumentError::BlankPattern. The error: '{error}'.",
                            error.type_id(),
                        )
                    }
                }
            }
        }

        mod with_invalid_filename {
            use std::any::Any;

            use super::*;

            #[test]
            fn pointing_to_a_directory() {
                let res = Command::build("pattern".to_owned(), RESOURCES_DIR.to_owned());
                check_result(res, PathType::Directory)
            }

            #[test]
            #[cfg(target_os = "unix")]
            fn pointing_to_unknown_typed_node() {
                let res = Command::build("pattern".to_owned(), "resources/unknown_file".to_owned());
                check_result(res, PathType::Unknown)
            }

            fn check_result(
                res: Result<Command, InvalidArgumentError>,
                waited_type_path: PathType,
            ) {
                assert!(
                    res.is_err(),
                    "Command should not be built with a path to {}.",
                    waited_type_path.build_part(),
                );

                let error = res.unwrap_err();

                match error {
                    InvalidArgumentError::NotAFile(_, file_type) => {
                        if file_type != format!("{waited_type_path}") {
                            panic!(
                                "Must be {}, not a {file_type}.",
                                waited_type_path.build_part()
                            )
                        }
                    }
                    _ => {
                        panic!(
                            "Invalid sub error {:?}, must be an \
                            InvalidArgumentError::NotAFile. The error: '{error}'.",
                            error.type_id(),
                        )
                    }
                }
            }

            #[test]
            fn pointing_to_a_not_existing_file() {
                let res = Command::build(
                    "pattern".to_owned(),
                    "resources/not_existing_file".to_owned(),
                );

                assert!(
                    res.is_err(),
                    "Command should not be built with a not existing file.",
                );

                let error = res.unwrap_err();

                match error {
                    InvalidArgumentError::FileNotFound(_) => {}
                    _ => {
                        panic!(
                            "Invalid sub error {:?}, must be an \
                            InvalidArgumentError::FileNotFound.",
                            error.type_id(),
                        )
                    }
                }
            }

            #[test]
            fn without_read_permission() {
                let res = Command::build("pattern".to_owned(), NOT_READABLE_FILE.to_owned());

                assert!(
                    res.is_err(),
                    "Command should not be built with a not-readable file.",
                );

                let error = res.unwrap_err();

                match error {
                    InvalidArgumentError::NotAReadableFile(..) => {}
                    _ => {
                        panic!(
                            "Invalid sub error {:?}, must be an \
                            InvalidArgumentError::NotAReadableFile.",
                            error.type_id(),
                        )
                    }
                }
            }
        }
    }

    mod try_from_iter {
        use super::*;

        #[test]
        fn with_all_args() {
            let args = vec![
                "test_program".to_string(),
                "pattern".to_string(),
                EXAMPLE_FILE.to_owned(),
            ];

            let res = Command::try_from_iter(args.into_iter());

            assert!(
                res.is_ok(),
                "Command should be built with a valid pattern and a valid \
                filename. The error: '{}'.",
                res.unwrap_err(),
            )
        }

        #[test]
        #[should_panic(expected = "Missing the executable name.")]
        fn without_args() {
            let args = vec![];
            Command::try_from_iter(args.into_iter()).unwrap();
        }

        #[test]
        fn without_pattern_and_filename() {
            let executable = "test_program";
            let args = vec![executable.to_owned()];

            let res = Command::try_from_iter(args.into_iter());

            assert!(
                res.is_err(),
                "Command should not be built without arguments."
            );

            let error = res.unwrap_err();

            assert_eq!(
                format!("{error}"),
                format!(
                    "Missing arguments. Call the script like: {executable} pattern \
                    filename"
                ),
                "Invalid error message, must be missing arguments. The error: \
                '{error}'.",
            )
        }

        #[test]
        fn without_filename() {
            let executable = "test_program";
            let args = vec![executable.to_owned(), "pattern".to_owned()];

            let res = Command::try_from_iter(args.into_iter());

            assert!(
                res.is_err(),
                "Command should not be built without arguments."
            );

            let error = res.unwrap_err();

            assert_eq!(
                format!("{error}"),
                format!(
                    "Missing arguments. Call the script like: {executable} pattern \
                    filename"
                ),
                "Invalid error message, must be missing arguments. The error: \
                '{error}'.",
            )
        }

        mod with_invalid_pattern {
            use super::*;

            #[test]
            fn as_a_blank_pattern() {
                let args = vec![
                    "test_program".to_owned(),
                    " ".to_owned(),
                    EXAMPLE_FILE.to_owned(),
                ];

                let res = Command::try_from_iter(args.into_iter());
                check_result(res, PatternType::Blank)
            }

            #[test]
            fn as_an_empty_pattern() {
                let args = vec![
                    "test_program".to_owned(),
                    "".to_owned(),
                    EXAMPLE_FILE.to_owned(),
                ];

                let res = Command::try_from_iter(args.into_iter());
                check_result(res, PatternType::Empty)
            }

            pub fn check_result(
                res: Result<Command, Box<dyn MiniGrepArgsError>>,
                pattern_type: PatternType,
            ) {
                assert!(
                    res.is_err(),
                    "The command should not be built with a {pattern_type} pattern.",
                );

                let error = res.unwrap_err();
                let error = format!("{error}");

                if !error.contains("Cannot have a blank searched text") {
                    panic!("The pattern is valid, why?");
                }
            }
        }

        mod with_invalid_filename {
            use std::any::Any;

            use super::*;

            #[test]
            fn pointing_to_a_directory() {
                let args = vec![
                    "test_program".to_owned(),
                    "pattern".to_owned(),
                    RESOURCES_DIR.to_owned(),
                ];

                let res = Command::try_from_iter(args.into_iter());
                check_result(res, PathType::Directory)
            }

            #[test]
            #[cfg(target_os = "unix")]
            fn pointing_to_unknown_typed_node() {
                let args = vec![
                    "test_program".to_owned(),
                    "pattern".to_owned(),
                    "resources/unknown_file".to_owned(),
                ];

                let res = Command::try_from_iter(args.into_iter());
                check_result(res, PathType::Unknown)
            }

            #[test]
            fn pointing_to_a_not_existing_file() {
                let args = vec![
                    "test_program".to_owned(),
                    "pattern".to_owned(),
                    "resources/not_existing_file".to_owned(),
                ];

                let res = Command::try_from_iter(args.into_iter());

                assert!(
                    res.is_err(),
                    "Command should not be built with a not existing file.",
                );

                let error = res.unwrap_err();
                let error = format!("{error}");

                if !error.contains("does not exist") {
                    panic!(
                        "Invalid sub error {:?}, must be an \
                        InvalidArgumentError::FileNotFound. The error: '{error}'.",
                        error.type_id(),
                    );
                }
            }

            #[test]
            fn without_read_permission() {
                let args = vec![
                    "test_program".to_owned(),
                    "pattern".to_owned(),
                    NOT_READABLE_FILE.to_owned(),
                ];

                let res = Command::try_from_iter(args.into_iter());

                assert!(
                    res.is_err(),
                    "Command should not be built with a not-readable file.",
                );

                let error = res.unwrap_err();
                let error = format!("{error}");

                if !error.contains("Cannot open the file") {
                    panic!(
                        "Invalid sub error {:?}, must be an \
                        InvalidArgumentError::NotAReadableFile. The error: '{error}'.",
                        error.type_id(),
                    );
                }
            }

            fn check_result(
                res: Result<Command, Box<dyn MiniGrepArgsError>>,
                waited_type_path: PathType,
            ) {
                assert!(
                    res.is_err(),
                    "Command should not be built with a path to {}.",
                    waited_type_path.build_part(),
                );

                let error = res.unwrap_err();
                let error = format!("{error}");

                if !error.contains("is not a file, it is") {
                    panic!(
                        "Invalid sub error {:?}, must be an \
                        InvalidArgumentError::NotAFile. The error: '{error}'.",
                        error.type_id(),
                    );
                }

                if !error.contains(&format!("{waited_type_path}")) {
                    panic!("Must be {}.", waited_type_path.build_part());
                }
            }
        }
    }

    mod search_from {
        use super::*;

        #[test]
        fn a_file_without_the_pattern() {
            let pattern = "pattern";
            let res = Command::build(pattern.to_owned(), EXAMPLE_FILE.to_owned());

            assert!(res.is_ok(), "An error occurred: '{}'.", res.unwrap_err());

            let content = res.unwrap().search();
            assert!(
                content.is_empty(),
                "The command MiniGrep found the pattern in the file {}",
                EXAMPLE_FILE,
            );
        }

        #[test]
        fn a_file_with_a_line_with_the_pattern() {
            let pattern = "test_data.txt";
            let res = Command::build(pattern.to_owned(), EXAMPLE_FILE.to_owned());

            assert!(res.is_ok(), "An error occurred: '{}'.", res.unwrap_err());

            let content = res.unwrap().search();
            assert!(
                !content.is_empty(),
                "The command MiniGrep did not find the pattern in the file {}",
                EXAMPLE_FILE,
            );

            assert_eq!(
                content.len(),
                1,
                "The command MiniGrep finds the pattern many times in the file {}. \
                Content: {:?}",
                EXAMPLE_FILE,
                content,
            )
        }

        #[test]
        fn a_file_with_many_lines_with_the_pattern() {
            let pattern = "Rust";
            let res = Command::build(pattern.to_owned(), EXAMPLE_FILE.to_owned());

            assert!(res.is_ok(), "An error occurred: '{}'.", res.unwrap_err());

            let content = res.unwrap().search();
            assert!(
                !content.is_empty(),
                "The command MiniGrep did not find the pattern in the file {}",
                EXAMPLE_FILE,
            );

            assert_ne!(
                content.len(),
                1,
                "The command MiniGrep finds the pattern one time in the file {}. \
                Content: {:?}",
                EXAMPLE_FILE,
                content,
            )
        }
    }
}
