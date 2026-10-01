use std::process::Command;

use rexpect::session::spawn_command;

#[test]
fn test_search_single_file() {
    let bin_path = assert_cmd::cargo::cargo_bin("kvdb");
    let cmd = Command::new(bin_path);
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
