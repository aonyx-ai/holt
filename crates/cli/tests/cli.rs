use assert_cmd::cargo::*;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn test_dir() -> PathBuf {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("holt-cli-test-{}-{}", std::process::id(), id));
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn help_shows_commands() {
    let mut cmd = cargo_bin_cmd!("holt");

    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("serve"))
        .stdout(predicate::str::contains("build"));
}

#[test]
fn serve_help_shows_options() {
    let mut cmd = cargo_bin_cmd!("holt");

    cmd.args(["serve", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--port"))
        .stdout(predicate::str::contains("--open"));
}

#[test]
fn build_help_shows_options() {
    let mut cmd = cargo_bin_cmd!("holt");

    cmd.args(["build", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--profile"));
}

#[test]
fn serve_accepts_port() {
    let mut cmd = cargo_bin_cmd!("holt");

    cmd.args(["serve", "--port", "3000"]);

    // Fails because trunk command not found, but args were parsed
    cmd.assert().failure();
}

#[test]
fn build_accepts_profile() {
    let mut cmd = cargo_bin_cmd!("holt");

    cmd.args(["build", "--profile", "release"]);

    // Fails because trunk command not found, but args were parsed
    cmd.assert().failure();
}

#[test]
fn runs_without_config_file() {
    let dir = test_dir();

    let mut cmd = cargo_bin_cmd!("holt");
    cmd.current_dir(&dir);
    cmd.args(["serve", "--port", "3000"]);

    // Fails because trunk not found, but config loading succeeded (no error about config)
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to load config").not());

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn loads_config_file() {
    let dir = test_dir();

    // Create a valid config file
    fs::write(
        dir.join("holt.toml"),
        r#"
[book]
path = "my-book"

[serve]
port = 3000
open = true
"#,
    )
    .unwrap();

    let mut cmd = cargo_bin_cmd!("holt");
    cmd.current_dir(&dir);
    cmd.args(["serve"]);

    // Fails because trunk not found, but config was loaded (no config error)
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to load config").not());

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn rejects_invalid_config() {
    let dir = test_dir();

    // Create an invalid TOML file
    fs::write(dir.join("holt.toml"), "this is not valid toml [[[").unwrap();

    let mut cmd = cargo_bin_cmd!("holt");
    cmd.current_dir(&dir);
    cmd.args(["serve"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to load config"));

    fs::remove_dir_all(&dir).ok();
}
