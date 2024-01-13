use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn die_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

#[test]
fn hello1() -> TestResult {
    let output_file = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(output_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}
#[test]
fn hello2() -> TestResult {
    let output_file = "tests/expected/hello2.txt";
    let expected = fs::read_to_string(output_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
