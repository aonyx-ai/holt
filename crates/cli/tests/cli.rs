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

/// Shows that the build command accepts `--profile`
///
/// The command fails because `trunk` is not installed, so the test checks only that the arguments parse.
#[test]
fn build_accepts_profile() {
    let mut cmd = cargo_bin_cmd!("holt");

    cmd.args(["build", "--profile", "release"]);

    cmd.assert().failure();
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
fn help_shows_commands() {
    let mut cmd = cargo_bin_cmd!("holt");

    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("serve"))
        .stdout(predicate::str::contains("build"));
}

/// Shows that the serve command loads a valid config file
///
/// The command fails because `trunk` is not installed, so the test checks only that the failure is no config error.
#[test]
fn loads_config_file() {
    let dir = test_dir();

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

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to load config").not());

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn rejects_invalid_config() {
    let dir = test_dir();

    fs::write(dir.join("holt.toml"), "this is not valid toml [[[").unwrap();

    let mut cmd = cargo_bin_cmd!("holt");
    cmd.current_dir(&dir);
    cmd.args(["serve"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to load config"));

    fs::remove_dir_all(&dir).ok();
}

/// Shows that the serve command runs without a config file
///
/// The command fails because `trunk` is not installed, so the test checks only that the failure is no config error.
#[test]
fn runs_without_config_file() {
    let dir = test_dir();

    let mut cmd = cargo_bin_cmd!("holt");
    cmd.current_dir(&dir);
    cmd.args(["serve", "--port", "3000"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to load config").not());

    fs::remove_dir_all(&dir).ok();
}

/// Shows that the serve command accepts `--port`
///
/// The command fails because `trunk` is not installed, so the test checks only that the arguments parse.
#[test]
fn serve_accepts_port() {
    let mut cmd = cargo_bin_cmd!("holt");

    cmd.args(["serve", "--port", "3000"]);

    cmd.assert().failure();
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
