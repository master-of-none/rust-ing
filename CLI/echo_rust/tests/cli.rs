use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

const OUTFILE: &str = "tests/expected/";
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo_rust")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

// #[test]
// fn runs() -> TestResult {
//     let mut cmd = Command::cargo_bin("echo_rust")?;
//     cmd.arg("hello").assert().success();
//     Ok(())
// }

// #[test]
// fn hello1() -> TestResult {
//     let outfile = "tests/expected/hello1.txt";
//     let expected = fs::read_to_string(outfile)?;
//     let mut cmd = Command::cargo_bin("echo_rust")?;
//     cmd.arg("Hello there").assert().success().stdout(expected);
//     Ok(())
// }

// #[test]
// fn hello2() -> TestResult {
//     let outfile = "tests/expected/hello2.txt";
//     let expected = fs::read_to_string(outfile)?;
//     let mut cmd = Command::cargo_bin("echo_rust")?;

//     cmd.args(vec!["Hello", "there"])
//         .assert()
//         .success()
//         .stdout(expected);
//     Ok(())
// }

// Since there are 4 files, must write hepler function and pass files

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echo_rust")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], &(format!("{}hello1.txt", OUTFILE)))
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], &(format!("{}hello2.txt", OUTFILE)))
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(
        &["Hello there", "-n"],
        &(format!("{}hello1.n.txt", OUTFILE)),
    )
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(
        &["-n", "Hello", "there"],
        &(format!("{}hello2.n.txt", OUTFILE)),
    )
}
