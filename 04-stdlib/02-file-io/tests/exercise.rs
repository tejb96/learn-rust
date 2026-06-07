// DO NOT EDIT — implement the solution in src/lib.rs

use file_io::{grep_lines, read_nonempty_lines, write_lines};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn write_and_read_lines() {
    let mut file = NamedTempFile::new().unwrap();
    let path = file.path().to_path_buf();
    write_lines(&path, &["alpha", "beta"]).unwrap();
    assert_eq!(
        read_nonempty_lines(&path).unwrap(),
        vec!["alpha".to_string(), "beta".to_string()]
    );
}

#[test]
fn read_skips_blank() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "hello").unwrap();
    writeln!(file).unwrap();
    writeln!(file, "  world  ").unwrap();
    let path = file.path().to_path_buf();
    assert_eq!(
        read_nonempty_lines(&path).unwrap(),
        vec!["hello".to_string(), "world".to_string()]
    );
}

#[test]
fn grep_finds_matches() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "error: disk").unwrap();
    writeln!(file, "info: ok").unwrap();
    writeln!(file, "error: net").unwrap();
    let path = file.path().to_path_buf();
    let got = grep_lines(&path, "error").unwrap();
    assert_eq!(got.len(), 2);
}
