use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::predicate;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("log-analyzer");

    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: FileNotExists"));

    Ok(())
}

#[test]
fn file_fully_valid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("log-analyzer");

    cmd.arg("tests/data/integration_example.logs");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("INFO: 1\nWARN: 1\nERROR: 1"));

    Ok(())
}

#[test]
fn file_partially_valid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("log-analyzer");

    cmd.arg("tests/data/integration_example_partially_valid.logs");

    println!("{}", cmd.assert());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("INFO: 1\nWARN: 1\nERROR: 0"))
        .stderr(predicate::str::contains("Unparsable logs: 1"));

    Ok(())
}

#[test]
fn file_fully_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("log-analyzer");

    cmd.arg("tests/data/integration_example_fully_invalid.logs");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("INFO: 0\nWARN: 0\nERROR: 0"))
        .stderr(predicate::str::contains("Unparsable logs: 3"));

    Ok(())
}
