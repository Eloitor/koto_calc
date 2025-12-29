use assert_cmd::Command;
use predicates::prelude::*;

fn bin() -> Command {
    Command::cargo_bin("koto_calc").expect("binary build")
}

#[test]
fn help_flag_displays_usage() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("USAGE"));
}

#[test]
fn version_flag_reports_package_version() {
    bin().arg("--version").assert().success().stdout(
        predicates::str::contains(format!("koto_calc {}", env!("CARGO_PKG_VERSION")))
            .and(predicates::str::contains("(koto "))
            .and(predicates::str::contains("algebraeon ")),
    );
}

#[test]
fn eval_mode_runs_inline_scripts() {
    bin()
        .args(["--eval", "print(21 * 2)"])
        .assert()
        .success()
        .stdout(predicates::str::contains("42"));
}

#[test]
fn print_config_renders_default_export_map() {
    bin()
        .arg("--print_config")
        .assert()
        .success()
        .stdout(predicates::str::contains("export").and(predicates::str::contains("repl")));
}

#[test]
fn koto_script_algebraeon_tests_pass() {
    let mut cmd = bin();
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd.args(["--tests", "tests/koto/algebraeon_tests.koto"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Algebraeon Koto tests passed"));
}
