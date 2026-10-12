use std::{path::PathBuf, process::Command};

use rexpect::session::spawn_command;

fn test_full_create_get_delete_exit_interaction(cmd: Command) {
    let mut session = spawn_command(cmd, Some(1000)).unwrap();
    assert!(session.exp_string("kvdb> ").is_ok());

    assert!(session.send_line("get key").is_ok());
    assert!(session.exp_string("NoSuchKey\r\nkvdb> ").is_ok());

    assert!(session.send_line("set key val").is_ok());
    assert!(session.exp_string("Ok\r\nkvdb> ").is_ok());

    assert!(session.send_line("get key").is_ok());
    assert!(session.exp_string("val\r\nkvdb> ").is_ok());

    assert!(session.send_line("delete key").is_ok());
    assert!(session.exp_string("val\r\nkvdb> ").is_ok());

    assert!(session.send_line("get key").is_ok());
    assert!(session.exp_string("NoSuchKey\r\nkvdb> ").is_ok());

    assert!(session.send_line("exit").is_ok());
    assert!(session.exp_eof().is_ok());
}

#[test]
fn test_in_memory_db_core_suite() {
    let bin_path = assert_cmd::cargo::cargo_bin("kvdb");
    let mut cmd = Command::new(bin_path);
    cmd.arg("in-memory");
    test_full_create_get_delete_exit_interaction(cmd);
}

#[test]
fn test_file_based_db_core_suite() {
    let tmpfile = tempfile::NamedTempFile::new().unwrap();
    let bin_path = assert_cmd::cargo::cargo_bin("kvdb");
    let mut cmd = Command::new(bin_path);
    cmd.args(["file-based", tmpfile.path().to_str().unwrap()]);
    test_full_create_get_delete_exit_interaction(cmd);
}

#[test]
fn test_file_based_db_no_create_flag_and_file_exists() {
    let tmpfile = tempfile::NamedTempFile::new().unwrap();
    let bin_path = assert_cmd::cargo::cargo_bin("kvdb");
    let mut cmd = Command::new(bin_path);
    cmd.args(["file-based", tmpfile.path().to_str().unwrap()]);
    let mut session = spawn_command(cmd, Some(1000)).unwrap();
    match session.exp_string("kvdb> ") {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e);
        }
    }
    assert!(session.send_line("exit").is_ok());
    assert!(session.exp_eof().is_ok());
}

#[test]
fn test_file_based_db_no_create_flag_and_file_not_exists() {
    let tmpdir = tempfile::tempdir().unwrap();
    let tmpfile = tmpdir.path().join("tempfile.db");
    let bin_path = assert_cmd::cargo::cargo_bin("kvdb");
    let mut cmd = Command::new(bin_path);
    cmd.args(["file-based", tmpfile.to_str().unwrap()]);
    let mut session = spawn_command(cmd, Some(1000)).unwrap();
    assert!(session.exp_string("No such file or directory").is_ok());
    assert!(session.exp_eof().is_ok());
}

#[test]
fn test_file_based_db_create_flag_and_file_not_exists() {
    let tmpdir = tempfile::tempdir().unwrap();
    let tmpfile = tmpdir.path().join("tempfile.db");
    let bin_path = assert_cmd::cargo::cargo_bin("kvdb");
    let mut cmd = Command::new(bin_path);
    cmd.args(["file-based", tmpfile.to_str().unwrap(), "--create"]);
    let mut session = spawn_command(cmd, Some(1000)).unwrap();
    match session.exp_string("kvdb> ") {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e);
        }
    }
    assert!(session.send_line("exit").is_ok());
    assert!(session.exp_eof().is_ok());
}

#[test]
fn test_file_based_db_create_flag_and_file_exists() {
    let tmpfile = PathBuf::from("tests/data/file.db");
    let bin_path = assert_cmd::cargo::cargo_bin("kvdb");
    let mut cmd = Command::new(bin_path);
    cmd.args(["file-based", tmpfile.to_str().unwrap(), "--create"]);
    let mut session = spawn_command(cmd, Some(1000)).unwrap();
    match session.exp_string("kvdb> ") {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e);
        }
    }
    assert!(session.send_line("get test").is_ok());
    assert!(session.exp_string("value\r\nkvdb> ").is_ok());
    assert!(session.send_line("exit").is_ok());
    assert!(session.exp_eof().is_ok());
}
