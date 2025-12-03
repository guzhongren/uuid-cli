use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn generate_v7_default() {
    let mut cmd = cargo_bin_cmd!("uuid-cli");
    cmd.assert().success().stdout(
        predicate::str::is_match(
            r"^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[0-9a-f]{4}-[0-9a-f]{12}\n$",
        )
        .unwrap(),
    );
}

#[test]
fn generate_no_hyphen() {
    let mut cmd = cargo_bin_cmd!("uuid-cli");
    cmd.arg("--no-hyphen");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]{32}\n$").unwrap());
}

#[test]
fn generate_braced_upper() {
    let mut cmd = cargo_bin_cmd!("uuid-cli");
    cmd.args(["--braced", "--upper"]);
    cmd.assert().success().stdout(
        predicate::str::is_match(
            r"^\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\}\n$",
        )
        .unwrap(),
    );
}

#[test]
fn v5_requires_name_and_generates_v5() {
    let mut cmd = cargo_bin_cmd!("uuid-cli");
    cmd.args(["-v", "5"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("requires --name"));

    let mut cmd2 = cargo_bin_cmd!("uuid-cli");
    cmd2.args(["-v", "5", "--name", "example"]);
    cmd2.assert().success().stdout(
        predicate::str::is_match(
            r"^[0-9a-f]{8}-[0-9a-f]{4}-5[0-9a-f]{3}-[0-9a-f]{4}-[0-9a-f]{12}\n$",
        )
        .unwrap(),
    );
}

#[test]
fn count_generates_multiple_lines() {
    let output = cargo_bin_cmd!("uuid-cli")
        .args(["-n", "3"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let s = String::from_utf8(output.stdout).unwrap();
    assert_eq!(s.lines().count(), 3);
}
