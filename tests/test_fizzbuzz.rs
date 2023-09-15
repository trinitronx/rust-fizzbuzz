#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use once_cell::sync::Lazy;
    // use predicates::prelude::*;
    use rstest::*;
    use std::{fs::read_to_string, path::PathBuf, str::FromStr};

    const FIXTURES_DIR: &'static str = "tests/fixtures/";
    static INPUT_PATH: Lazy<PathBuf> = Lazy::new(|| {
        PathBuf::from_str(FIXTURES_DIR)
            .unwrap()
            .join("input")
            .join("input00.txt")
    });
    static EXPECTED_OUTPUT_PATH: Lazy<PathBuf> = Lazy::new(|| {
        PathBuf::from_str(FIXTURES_DIR)
            .unwrap()
            .join("output")
            .join("output00.txt")
    });

    #[fixture]
    pub fn expected_output() -> String {
        let s = read_to_string(EXPECTED_OUTPUT_PATH.as_path());
        match s {
            Ok(s) => s,
            _ => "".to_string(),
        }
    }

    #[rstest]
    fn should_success(expected_output: String) {
        // println!("Expected output: {expected_output}");
        let mut cmd = Command::cargo_bin("fizzbuzz").unwrap();
        // cmd.arg("N/A");
        let assert = cmd
            .pipe_stdin(INPUT_PATH.as_path())
            .expect(format!("Expected Input file not found: {:?}", INPUT_PATH.as_path()).as_str())
            .assert();

        // Check for output starts with
        assert
            .success()
            .stdout(predicates::str::starts_with(expected_output));

        // Alt check for exact match
        // if let Ok(output) = cmd.output() {
        //     assert_eq!(
        //         String::from_utf8(output.stdout.clone()).unwrap(),
        //         expected_output
        //     );
        // }
    }

    #[rstest]
    #[should_panic]
    fn should_fail(expected_output: String) {
        assert_eq!(expected_output, "".to_string());
    }
}
