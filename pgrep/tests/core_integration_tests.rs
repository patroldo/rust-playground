use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::predicate;

#[test]
fn test_search_single_file() {
    let mut cmd = cargo_bin_cmd!("pgrep");
    let args = ["INFO", "tests/data/simple_logs.logs"];
    cmd.args(args);
    cmd.assert().success().stdout(
        "2026-09-11T10:01:03Z INFO server started
2026-09-11T10:01:03Z INFO server has started\n",
    );
}

#[test]
fn test_file_not_exists() {
    let mut cmd = cargo_bin_cmd!("pgrep");
    let args = ["INFO", "file/that/dont/exists"];
    cmd.args(args);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
}

#[test]
fn test_search_single_file_multiple_lines() {
    let mut cmd = cargo_bin_cmd!("pgrep");
    let args = ["INFO", "tests/data/simple_logs_4.logs"];
    cmd.args(args);
    cmd.assert().success().stdout(
        "2026-09-11T10:01:03Z INFO server started
2026-09-11T10:01:03Z INFO server checked
2026-09-11T10:01:03Z INFO server connecting\n",
    );
}

#[test]
fn test_search_multiple_files() {
    let mut cmd = cargo_bin_cmd!("pgrep");
    let args = [
        "INFO",
        "tests/data/simple_logs.logs",
        "tests/data/simple_logs_2.logs",
    ];
    cmd.args(args);
    cmd.assert().success().stdout(
        "tests/data/simple_logs.logs:2026-09-11T10:01:03Z INFO server started
tests/data/simple_logs.logs:2026-09-11T10:01:03Z INFO server has started
tests/data/simple_logs_2.logs:INFO try to connect\n",
    );
}

#[test]
fn test_search_multiple_files_some_not_exist() {
    let mut cmd = cargo_bin_cmd!("pgrep");
    let args = [
        "INFO",
        "tests/data/simple_logs.logs",
        "tests/data/simple_logs_2.logs",
        "file/that/dont/exists",
    ];
    cmd.args(args);
    cmd.assert()
        .success()
        .stdout(
            "tests/data/simple_logs.logs:2026-09-11T10:01:03Z INFO server started
tests/data/simple_logs.logs:2026-09-11T10:01:03Z INFO server has started
tests/data/simple_logs_2.logs:INFO try to connect\n",
        )
        .stderr(predicate::str::contains(
            "file/that/dont/exists:No such file or directory",
        ));
}
