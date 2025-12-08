use ::assert_cmd::Command;
use ::predicates::prelude::*;

/// Helper function to create a command instance for the CLI binary.
/// Assumes the binary name matches the package name 'weather'.
fn weather_cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_weather"))
}

#[test]
fn test_help_command() {
    let mut cmd = weather_cli();

    // Run `weather --help` and assert that it runs successfully
    // and contains expected usage information.
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("provider"))
        .stdout(predicate::str::contains("alias"));
}

#[test]
fn test_version_flag() {
    let mut cmd = weather_cli();

    // Run `weather --version`
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("weather"));
}

#[test]
fn test_get_weather_mock_default() {
    let mut cmd = weather_cli();

    // Test the `get` command using the Mock provider.
    // We explicitly set the provider to 'mock' to avoid needing a config file with keys.
    cmd.arg("get")
        .arg("London")
        .arg("--provider")
        .arg("mock")
        .assert()
        .success()
        .stdout(predicate::str::contains("Fetching weather from 'MockWeather'"))
        .stdout(predicate::str::contains("Mock City"))
        .stdout(predicate::str::contains("Mock Country"))
        .stdout(predicate::str::contains("Sunny (Mock)"));
}

#[test]
fn test_get_weather_with_date_mock() {
    let mut cmd = weather_cli();

    // Test requesting weather for a specific date using the Mock provider.
    cmd.arg("get")
        .arg("Paris")
        .arg("--date")
        .arg("2023-12-25")
        .arg("--provider")
        .arg("mock")
        .assert()
        .success()
        .stdout(predicate::str::contains("Fetching weather from 'MockWeather'"))
        .stdout(predicate::str::contains("Paris"));
}

#[test]
fn test_provider_list() {
    let mut cmd = weather_cli();

    // Test the `provider --list` command.
    cmd.arg("provider")
        .arg("--list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Weather providers:"))
        .stdout(predicate::str::contains("mock"))
        .stdout(predicate::str::contains("ow"))
        .stdout(predicate::str::contains("wa"));
}

#[test]
fn test_fail_unknown_provider() {
    let mut cmd = weather_cli();

    // Expect failure when using a non-existent provider.
    cmd.arg("get")
        .arg("Berlin")
        .arg("--provider")
        .arg("unknown_provider")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown provider"));
}

#[test]
fn test_fail_missing_address() {
    let mut cmd = weather_cli();

    // Expect failure when no address and no default alias are configured.
    // This assumes the test environment doesn't have a config file set up yet.
    cmd.arg("get")
        .arg("--provider")
        .arg("mock")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No address specified"));
}