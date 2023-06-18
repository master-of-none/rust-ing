use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn does_not_work() {
    let mut cmd = Command::cargo_bin("echo_rust2").unwrap();
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn works_correctly() {
    let mut cmd = Command::cargo_bin("echo_rust2").unwrap();
    cmd.arg("hello").assert().success();
}
