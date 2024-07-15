pub use self::command::Command;

mod command;

mod errors;

#[cfg(test)]
mod e2e_tests {
    use std::process::Command as Cmd;

    use rstest::*;
    use rstest_reuse::{self, *};

    use crate::mini_grep::Command;

    const RESOURCES_DIR: &str = "resources";
    const EXAMPLE_FILE: &str = "resources/example.txt";

    #[cfg(windows)]
    const NOT_READABLE_FILE: &str = "resources/not_readable_file.txt";
    #[cfg(unix)]
    const NOT_READABLE_FILE: &str = "resources/not_readable_unix_file";

    #[fixture]
    fn mini_grep_cmd(
        #[default("pattern")] pattern: &'static str,
        #[default(EXAMPLE_FILE)] filename: &'static str,
        #[default(false)] give_ignore_case: bool,
        #[default("0")] ignore_case: &'static str,
    ) -> Cmd {
        let mut cargo = Cmd::new("cargo");
        cargo.args(["run", "--", pattern, filename]);

        if give_ignore_case {
            cargo.env(Command::IGNORE_CASE_ENV_NAME, ignore_case);
        }

        cargo
    }

    #[template]
    #[rstest]
    #[case::case_sensitive("0", true)]
    #[case::case_sensitive("false", true)]
    #[case::case_sensitive("tata", true)]
    #[case::case_sensitive("FALSE", true)]
    #[case::case_sensitive("alpha", true)]
    #[case::case_sensitive("", false)]
    #[case::case_sensitive("  ", false)]
    #[case::case_insensitive("1", true)]
    #[case::case_insensitive("true", true)]
    #[case::case_insensitive("TRUE", true)]
    #[case::case_insensitive("TrUe", true)]
    fn case_sensitive_test_cases(#[case] case_mode: &'static str, #[case] give_case_mode: bool) {}

    mod with_an_invalid_pattern {
        use super::*;

        #[apply(case_sensitive_test_cases)]
        fn as_an_empty_pattern(
            #[case] _case_mode: &'static str,
            #[case] _give_case_mode: bool,
            #[with("", "", _give_case_mode, _case_mode)] mut mini_grep_cmd: Cmd,
        ) {
            let output = match mini_grep_cmd.output() {
                Ok(output) => output,
                Err(error) => {
                    panic!("Error during the spawn of the command mini-grep, the error: '{error}'.")
                }
            };

            let stdout = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => {
                    panic!("Error during the string conversion of stdout. The error: '{error}'.")
                }
            };

            let stderr = match String::from_utf8(output.stderr) {
                Ok(stderr) => stderr,
                Err(error) => {
                    panic!("Error during the string conversion of stderr. The error: '{error}'.")
                }
            };

            assert!(stdout.is_empty(), "Standard output: '{stdout}'.");
            assert!(!stderr.is_empty(), "Standard error output: '{stderr}'.");

            assert!(
                stderr.contains("Cannot have a blank searched text ''."),
                "Bad error in stderr: '{stderr}'.",
            );
        }

        #[apply(case_sensitive_test_cases)]
        fn as_a_blank_pattern(
            #[case] _case_mode: &'static str,
            #[case] _give_case_mode: bool,
            #[with("  ", "", _give_case_mode, _case_mode)] mut mini_grep_cmd: Cmd,
        ) {
            let output = match mini_grep_cmd.output() {
                Ok(output) => output,
                Err(error) => {
                    panic!("Error during the spawn of the command mini-grep, the error: '{error}'.")
                }
            };

            let stdout = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => {
                    panic!("Error during the string conversion of stdout. The error: '{error}'.")
                }
            };

            let stderr = match String::from_utf8(output.stderr) {
                Ok(stderr) => stderr,
                Err(error) => {
                    panic!("Error during the string conversion of stderr. The error: '{error}'.")
                }
            };

            assert!(stdout.is_empty(), "Standard output: '{stdout}'.");
            assert!(!stderr.is_empty(), "Standard error output: '{stderr}'.");

            assert!(
                stderr.contains("Cannot have a blank searched text '  '."),
                "Bad error in stderr: '{stderr}'.",
            );
        }
    }

    mod with_an_invalid_path {
        use super::*;

        const PATTERN: &str = "pattern";

        #[apply(case_sensitive_test_cases)]
        fn pointing_to_a_directory(
            #[case] _case_mode: &'static str,
            #[case] _give_case_mode: bool,
            #[with(PATTERN, RESOURCES_DIR, _give_case_mode, _case_mode)] mut mini_grep_cmd: Cmd,
        ) {
            let output = match mini_grep_cmd.output() {
                Ok(output) => output,
                Err(error) => {
                    panic!("Error during the spawn of the command mini-grep, the error: '{error}'.")
                }
            };

            let stdout = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => {
                    panic!("Error during the string conversion of stdout. The error: '{error}'.")
                }
            };

            let stderr = match String::from_utf8(output.stderr) {
                Ok(stderr) => stderr,
                Err(error) => {
                    panic!("Error during the string conversion of stderr. The error: '{error}'.")
                }
            };

            assert!(stdout.is_empty(), "Standard output: '{stdout}'.");
            assert!(!stderr.is_empty(), "Standard error output: '{stderr}'.");

            assert!(
                stderr.contains(&format!(
                    "{RESOURCES_DIR}' is not a file, it is a directory.",
                )),
                "Bad error in stderr: '{stderr}'.",
            );
        }

        #[cfg(unix)]
        #[apply(case_sensitive_test_cases)]
        fn pointing_to_unknown_typed_node(
            #[case] _case_mode: &'static str,
            #[case] _give_case_mode: bool,
            #[with(PATTERN, "", _give_case_mode, _case_mode)] mut mini_grep_cmd: Cmd,
        ) {
            let output = match mini_grep_cmd.output() {
                Ok(output) => output,
                Err(error) => {
                    panic!("Error during the spawn of the command mini-grep, the error: '{error}'.")
                }
            };

            let stdout = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => {
                    panic!("Error during the string conversion of stdout. The error: '{error}'.")
                }
            };

            let stderr = match String::from_utf8(output.stderr) {
                Ok(stderr) => stderr,
                Err(error) => {
                    panic!("Error during the string conversion of stderr. The error: '{error}'.")
                }
            };

            assert!(stdout.is_empty(), "Standard output: '{stdout}'.");
            assert!(!stderr.is_empty(), "Standard error output: '{stderr}'.");

            assert!(
                stderr.contains("Cannot have a blank searched text ''."),
                "Bad error in stderr: '{stderr}'.",
            );
        }

        #[apply(case_sensitive_test_cases)]
        fn pointing_to_a_not_existing_file(
            #[case] _case_mode: &'static str,
            #[case] _give_case_mode: bool,
            #[with(PATTERN, "resources/not_existing_file", _give_case_mode, _case_mode)]
            mut mini_grep_cmd: Cmd,
        ) {
            let output = match mini_grep_cmd.output() {
                Ok(output) => output,
                Err(error) => {
                    panic!("Error during the spawn of the command mini-grep, the error: '{error}'.")
                }
            };

            let stdout = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => {
                    panic!("Error during the string conversion of stdout. The error: '{error}'.")
                }
            };

            let stderr = match String::from_utf8(output.stderr) {
                Ok(stderr) => stderr,
                Err(error) => {
                    panic!("Error during the string conversion of stderr. The error: '{error}'.")
                }
            };

            assert!(stdout.is_empty(), "Standard output: '{stdout}'.");
            assert!(!stderr.is_empty(), "Standard error output: '{stderr}'.");

            assert!(
                stderr.contains("The file 'resources/not_existing_file' does not exist."),
                "Bad error in stderr: '{stderr}'.",
            );
        }

        #[apply(case_sensitive_test_cases)]
        fn pointing_to_a_file_without_read_permission(
            #[case] _case_mode: &'static str,
            #[case] _give_case_mode: bool,
            #[with(PATTERN, NOT_READABLE_FILE, _give_case_mode, _case_mode)] mut mini_grep_cmd: Cmd,
        ) {
            let output = match mini_grep_cmd.output() {
                Ok(output) => output,
                Err(error) => {
                    panic!("Error during the spawn of the command mini-grep, the error: '{error}'.")
                }
            };

            let stdout = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => {
                    panic!("Error during the string conversion of stdout. The error: '{error}'.")
                }
            };

            let stderr = match String::from_utf8(output.stderr) {
                Ok(stderr) => stderr,
                Err(error) => {
                    panic!("Error during the string conversion of stderr. The error: '{error}'.")
                }
            };

            assert!(stdout.is_empty(), "Standard output: '{stdout}'.");
            assert!(!stderr.is_empty(), "Standard error output: '{stderr}'.");

            assert!(
                stderr.contains(&format!(
                    "Cannot open the file '{NOT_READABLE_FILE}', due to this error",
                )),
                "Bad error in stderr: '{stderr}'.",
            );

            #[cfg(windows)]
            assert!(
                stderr.contains("(os error 5)"),
                "Bad error in stderr: '{stderr}'.",
            );
            #[cfg(unix)]
            todo!("Implement the error code check")
        }
    }

    mod with_a_valid_file {
        use std::ops::Not;

        use super::*;

        fn clear_useless_lines_from(stderr: String) -> String {
            String::from_iter(stderr.lines().filter(|line| {
                let trimmed_line = line.trim_start();
                (trimmed_line.starts_with("Blocking")
                    || trimmed_line.starts_with("Running")
                    || trimmed_line.starts_with("Finished")
                    || trimmed_line.starts_with("Compiling"))
                .not()
            }))
        }

        #[apply(case_sensitive_test_cases)]
        fn without_the_pattern(
            #[case] _case_mode: &'static str,
            #[case] give_case_mode: bool,
            #[with("pattern", EXAMPLE_FILE, give_case_mode, _case_mode)] mut mini_grep_cmd: Cmd,
        ) {
            let output = match mini_grep_cmd.output() {
                Ok(output) => output,
                Err(error) => {
                    panic!("Error during the spawn of the command mini-grep, the error: '{error}'.")
                }
            };

            let stdout = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => {
                    panic!("Error during the string conversion of stdout. The error: '{error}'.")
                }
            };

            let stderr = clear_useless_lines_from(String::from_utf8(output.stderr).unwrap_or_else(
                |error| {
                    panic!("Error during the string conversion of stderr. The error: '{error}'.")
                },
            ));

            assert!(!stdout.is_empty(), "Standard output: '{stdout}'.");

            if give_case_mode {
                assert!(stderr.is_empty(), "Standard error output: '{stderr}'.");
            } else {
                assert_eq!(
                    stderr,
                    "Error during the get of the variable 'IGNORE_CASE'. The error: \
                    'environment variable not found'.",
                    "Bad error when getting the environment variable {}.",
                    Command::IGNORE_CASE_ENV_NAME,
                );
            }

            let lines: Vec<_> = stdout.lines().collect();

            let first_line = match lines.first() {
                Some(line) => line,
                None => panic!("Missing the first line in stdout: '{stdout}'."),
            };

            assert!(
                first_line.starts_with(&format!(
                    "The file '{EXAMPLE_FILE}' does not contain any line with the case ",
                )),
                "The first line: '{first_line}' is invalid.",
            );
            assert!(
                first_line.ends_with("pattern 'pattern'."),
                "The first line: '{first_line}' finishes with bad words.",
            );

            if let Some(line) = lines.get(1) {
                panic!("A second line exists in stdout: '{line}'.")
            }
        }

        #[rstest]
        #[case::case_sensitive("0", true)]
        #[case::case_sensitive("false", true)]
        #[case::case_sensitive("tata", true)]
        #[case::case_sensitive("FALSE", true)]
        #[case::case_sensitive("alpha", true)]
        #[case::case_sensitive("", false)]
        #[case::case_sensitive("  ", false)]
        fn with_a_line_with_the_pattern(
            #[case] _case_mode: &'static str,
            #[case] give_case_mode: bool,
            #[with("test_data", EXAMPLE_FILE, give_case_mode, _case_mode)] mut mini_grep_cmd: Cmd,
        ) {
            let output = match mini_grep_cmd.output() {
                Ok(output) => output,
                Err(error) => {
                    panic!("Error during the spawn of the command mini-grep, the error: '{error}'.")
                }
            };

            let stdout = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => {
                    panic!("Error during the string conversion of stdout. The error: '{error}'.")
                }
            };

            let stderr = clear_useless_lines_from(String::from_utf8(output.stderr).unwrap_or_else(
                |error| {
                    panic!("Error during the string conversion of stderr. The error: '{error}'.")
                },
            ));

            assert!(!stdout.is_empty(), "Standard output: '{stdout}'.");

            if give_case_mode {
                assert!(stderr.is_empty(), "Standard error output: '{stderr}'.");
            } else {
                assert_eq!(
                    stderr,
                    "Error during the get of the variable 'IGNORE_CASE'. The error: \
                    'environment variable not found'.",
                    "Bad error when getting the environment variable {}.",
                    Command::IGNORE_CASE_ENV_NAME,
                );
            }

            let lines: Vec<_> = stdout.lines().collect();

            let first_line = match lines.first() {
                Some(line) => line,
                None => panic!("Missing the first line in stdout: '{stdout}'."),
            };

            assert!(
                first_line.starts_with(&format!(
                    "The file '{EXAMPLE_FILE}' contains these lines with the case",
                )),
                "The first line: '{first_line}' is invalid.",
            );
            assert!(
                first_line.ends_with("pattern 'test_data':"),
                "The first line: '{first_line}' finishes with bad words.",
            );

            let second_line = match lines.get(1) {
                Some(line) => line,
                None => panic!("Missing the second line in stdout: '{stdout}'."),
            };
            assert_eq!(
                second_line.to_string(),
                "1: # test_data.txt",
                "The found line is invalid for the file '{EXAMPLE_FILE}'."
            );

            if let Some(line) = lines.get(2) {
                panic!("A third line exists in stdout: '{line}'.")
            }
        }

        #[rstest]
        #[case::case_insensitive("1")]
        #[case::case_insensitive("true")]
        #[case::case_insensitive("TRUE")]
        #[case::case_insensitive("TrUe")]
        fn with_a_line_with_the_pattern_and_ignore_case(
            #[case] _case_mode: &'static str,
            #[with("TesT_DaTA", EXAMPLE_FILE, true, _case_mode)] mut mini_grep_cmd: Cmd,
        ) {
            let output = match mini_grep_cmd.output() {
                Ok(output) => output,
                Err(error) => {
                    panic!("Error during the spawn of the command mini-grep, the error: '{error}'.")
                }
            };

            let stdout = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => {
                    panic!("Error during the string conversion of stdout. The error: '{error}'.")
                }
            };

            let stderr = clear_useless_lines_from(String::from_utf8(output.stderr).unwrap_or_else(
                |error| {
                    panic!("Error during the string conversion of stderr. The error: '{error}'.")
                },
            ));

            assert!(!stdout.is_empty(), "Standard output: '{stdout}'.");
            assert!(stderr.is_empty(), "Standard error output: '{stderr}'.");

            let lines: Vec<_> = stdout.lines().collect();

            let first_line = match lines.first() {
                Some(line) => line,
                None => panic!("Missing the first line in stdout: '{stdout}'."),
            };

            assert!(
                first_line.starts_with(&format!(
                    "The file '{EXAMPLE_FILE}' contains these lines with the case",
                )),
                "The first line: '{first_line}' is invalid.",
            );
            assert!(
                first_line.ends_with("pattern 'TesT_DaTA':"),
                "The first line: '{first_line}' finishes with bad words.",
            );

            let second_line = match lines.get(1) {
                Some(line) => line,
                None => panic!("Missing the second line in stdout: '{stdout}'."),
            };
            assert_eq!(
                second_line.to_string(),
                "1: # test_data.txt".to_owned(),
                "The found line is invalid for the file '{EXAMPLE_FILE}'."
            );

            if let Some(line) = lines.get(2) {
                panic!("A third line exists in stdout: '{line}'.")
            }
        }

        #[rstest]
        #[case::case_sensitive("0", true)]
        #[case::case_sensitive("false", true)]
        #[case::case_sensitive("tata", true)]
        #[case::case_sensitive("FALSE", true)]
        #[case::case_sensitive("alpha", true)]
        #[case::case_sensitive("", false)]
        #[case::case_sensitive("  ", false)]
        fn with_many_line_with_the_pattern(
            #[case] _case_mode: &'static str,
            #[case] give_case_mode: bool,
            #[with("Rust", EXAMPLE_FILE, give_case_mode, _case_mode)] mut mini_grep_cmd: Cmd,
        ) {
            let output = match mini_grep_cmd.output() {
                Ok(output) => output,
                Err(error) => {
                    panic!("Error during the spawn of the command mini-grep, the error: '{error}'.")
                }
            };

            let stdout = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => {
                    panic!("Error during the string conversion of stdout. The error: '{error}'.")
                }
            };

            let stderr = clear_useless_lines_from(String::from_utf8(output.stderr).unwrap_or_else(
                |error| {
                    panic!("Error during the string conversion of stderr. The error: '{error}'.")
                },
            ));

            assert!(!stdout.is_empty(), "Standard output: '{stdout}'.");

            if give_case_mode {
                assert!(stderr.is_empty(), "Standard error output: '{stderr}'.");
            } else {
                assert_eq!(
                    stderr,
                    "Error during the get of the variable 'IGNORE_CASE'. The error: \
                    'environment variable not found'.",
                    "Bad error when getting the environment variable {}.",
                    Command::IGNORE_CASE_ENV_NAME,
                );
            }

            let mut lines: Vec<_> = stdout.lines().collect();

            let first_line = match lines.first() {
                Some(line) => line,
                None => panic!("Missing the first line in stdout: '{stdout}'."),
            };

            assert!(
                first_line.starts_with(&format!(
                    "The file '{EXAMPLE_FILE}' contains these lines with the case",
                )),
                "The first line: '{first_line}' is invalid.",
            );
            assert!(
                first_line.ends_with("pattern 'Rust':"),
                "The first line: '{first_line}' finishes with bad words.",
            );

            // Drop the first line
            lines.remove(0);

            let waited_lines = [
                "3: This is a Rust Rover file.",
                "4: RustRover is a very good tool built in Rust.",
                "5: If you like Rust, you'd love this tool.",
                "6: RustRover 2024.1.4 is the best version so far.",
                "7: Programming is fun especially with a tool like RustRover.",
            ];

            check_many_lines_are_good(lines.into_iter(), waited_lines.into_iter(), "Rust")
        }

        fn check_many_lines_are_good<'a, I, J>(lines: I, mut waited_lines: J, pattern: &str)
        where
            I: Iterator<Item = &'a str>,
            J: Iterator<Item = &'a str>,
        {
            for found_line in lines {
                let line = match waited_lines.next() {
                    Some(entry) => entry,
                    None => panic!("MiniGrep found too many lines for the pattern '{pattern}'."),
                };

                assert_eq!(
                    found_line.to_string(),
                    line,
                    "The found line is invalid for the file '{EXAMPLE_FILE}'. \
                    Waited line: '{line}'",
                );
            }

            if let Some(line) = waited_lines.next() {
                panic!("MiniGrep found no enough lines for the pattern '{pattern}': '{line}'.")
            }
        }

        #[rstest]
        #[case::case_insensitive("1")]
        #[case::case_insensitive("true")]
        #[case::case_insensitive("TRUE")]
        #[case::case_insensitive("TrUe")]
        fn with_many_line_with_the_pattern_and_ignore_case(
            #[case] _case_mode: &'static str,
            #[with("rUsT", EXAMPLE_FILE, true, _case_mode)] mut mini_grep_cmd: Cmd,
        ) {
            let output = match mini_grep_cmd.output() {
                Ok(output) => output,
                Err(error) => {
                    panic!("Error during the spawn of the command mini-grep, the error: '{error}'.")
                }
            };

            let stdout = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => {
                    panic!("Error during the string conversion of stdout. The error: '{error}'.")
                }
            };

            let stderr = clear_useless_lines_from(String::from_utf8(output.stderr).unwrap_or_else(
                |error| {
                    panic!("Error during the string conversion of stderr. The error: '{error}'.")
                },
            ));

            assert!(!stdout.is_empty(), "Standard output: '{stdout}'.");
            assert!(stderr.is_empty(), "Standard error output: '{stderr}'.");

            let mut lines: Vec<_> = stdout.lines().collect();

            let first_line = match lines.first() {
                Some(line) => line,
                None => panic!("Missing the first line in stdout: '{stdout}'."),
            };

            assert!(
                first_line.starts_with(&format!(
                    "The file '{EXAMPLE_FILE}' contains these lines with the case",
                )),
                "The first line: '{first_line}' is invalid.",
            );
            assert!(
                first_line.ends_with("pattern 'rUsT':"),
                "The first line: '{first_line}' finishes with bad words.",
            );

            // Drop the first line
            lines.remove(0);

            let waited_lines = [
                "3: This is a Rust Rover file.",
                "4: RustRover is a very good tool built in Rust.",
                "5: If you like Rust, you'd love this tool.",
                "6: RustRover 2024.1.4 is the best version so far.",
                "7: Programming is fun especially with a tool like RustRover.",
            ];

            check_many_lines_are_good(lines.into_iter(), waited_lines.into_iter(), "rUsT")
        }
    }
}
