use assert_cmd::prelude::*; // add methods on commands
use assert_fs::prelude::*; // add methods on files
use predicates::prelude::*; // used for writing assertions
use std::process::Command; // write tests for CLI commands

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foobar").arg("lorem");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_a_match() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("lorem ipsum\ndolor sit amet")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("lorem").arg(file.path());

    cmd.assert().success().stdout("lorem ipsum\n");

    Ok(())
}

#[test]
fn find_multiple_matches() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("lorem ipsum\ndolor sit amet\nlorems")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("lorem").arg(file.path());

    cmd.assert().success().stdout("lorem ipsum\nlorems\n");

    Ok(())
}

#[test]
fn find_no_matches() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("lorem ipsum\ndolor sit amet")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foo").arg(file.path());

    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

#[test]
fn empty_string_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("lorem ipsum\ndolor sit amet")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("").arg(file.path());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Pattern is empty"));

    Ok(())
}
