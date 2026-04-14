use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn lists_rules() {
    Command::cargo_bin("thin")
        .unwrap()
        .arg("--list-rules")
        .assert()
        .success()
        .stdout(predicate::str::contains("thin.em-dash.cluster"))
        .stdout(predicate::str::contains("thin.filler.leverages"));
}

#[test]
fn shows_single_rule() {
    Command::cargo_bin("thin")
        .unwrap()
        .args(["--rule", "thin.filler.leverages"])
        .assert()
        .success()
        .stdout(predicate::str::contains("leverages"));
}

#[test]
fn lints_ai_fixture_and_exits_nonzero() {
    let out = Command::cargo_bin("thin")
        .unwrap()
        .args(["--no-color", "tests/fixtures/ai-generated-readme.md"])
        .assert();
    out.failure();
}

#[test]
fn lints_clean_fixture_and_exits_zero() {
    Command::cargo_bin("thin")
        .unwrap()
        .args(["--no-color", "tests/fixtures/clean-readme.md"])
        .assert()
        .success();
}

#[test]
fn json_format_outputs_valid_json() {
    let output = Command::cargo_bin("thin")
        .unwrap()
        .args([
            "--no-color",
            "--format",
            "json",
            "tests/fixtures/corporate.md",
        ])
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&s).expect("invalid json");
    assert!(v.is_array());
}

#[test]
fn stdin_mode() {
    Command::cargo_bin("thin")
        .unwrap()
        .args(["--no-color", "--stdin"])
        .write_stdin("hello world.\n")
        .assert()
        .success();
}

#[test]
fn profile_relaxed_skips_corporate() {
    Command::cargo_bin("thin")
        .unwrap()
        .args([
            "--no-color",
            "--profile",
            "relaxed",
            "tests/fixtures/corporate.md",
        ])
        .assert()
        .success(); // no errors in relaxed
}
