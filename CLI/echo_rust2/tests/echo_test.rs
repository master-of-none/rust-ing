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
fn hello1() {
    let expected = "Hello there \n";
    let mut cmd = Command::cargo_bin("echo_rust2").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}

#[test]
fn hello2() {
    let expected = "Hello there";
    let mut cmd = Command::cargo_bin("echo_rust2").unwrap();
    cmd.args(vec!["-n", "Hello", "there"])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn hello3() {
    let expected = "Hello there";
    let mut cmd = Command::cargo_bin("echo_rust2").unwrap();
    cmd.args(vec!["Hello", "there", "-n"])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn hello4() {
    let expected = "Hello -n there -n";
    let mut cmd = Command::cargo_bin("echo_rust2").unwrap();
    cmd.args(vec!["-n", "Hello", "-n", "there", "-n"])
        .assert()
        .success()
        .stdout(expected);
}
