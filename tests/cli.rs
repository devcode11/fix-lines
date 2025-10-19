use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use fix_lines::DEFAULT_LINE_ENDING;
use predicates::prelude::*;
use std::process::Command;

type TestError = Result<(), Box<dyn std::error::Error>>;

#[test]
fn file_does_not_exist() -> TestError {
    let mut cmd = Command::cargo_bin("fix-lines")?;

    cmd.arg("non_existing_file.txt");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to open file for read"));

    Ok(())
}

#[test]
fn trim_trailing_empty_lines() -> TestError {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("Some text\n \t  \r \n   \n")?;
    let mut cmd = Command::cargo_bin("fix-lines")?;

    cmd.arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    let updated_content = std::fs::read_to_string(file.path())?;
    assert_eq!(updated_content, format!("Some text{}", DEFAULT_LINE_ENDING));

    Ok(())
}
