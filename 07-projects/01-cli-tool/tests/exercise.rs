// DO NOT EDIT — implement the solution in src/lib.rs

use cli_tool::{filter_by_level, format_json, parse_line, LogEntry};

#[test]
fn parse_line_info() {
    let entry = parse_line("INFO server started").unwrap();
    assert_eq!(
        entry,
        LogEntry {
            level: "INFO".into(),
            message: "server started".into(),
        }
    );
}

#[test]
fn parse_line_error() {
    let entry = parse_line("ERROR disk full on /data").unwrap();
    assert_eq!(entry.level, "ERROR");
    assert_eq!(entry.message, "disk full on /data");
}

#[test]
fn parse_line_invalid() {
    assert!(parse_line("   ").is_err());
}

#[test]
fn filter_by_level() {
    let entries = vec![
        LogEntry {
            level: "INFO".into(),
            message: "a".into(),
        },
        LogEntry {
            level: "ERROR".into(),
            message: "b".into(),
        },
    ];
    let filtered = filter_by_level(&entries, "error");
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].message, "b");
}

#[test]
fn format_json_output() {
    let entries = vec![LogEntry {
        level: "INFO".into(),
        message: "ok".into(),
    }];
    let json = format_json(&entries).unwrap();
    assert!(json.contains("INFO"));
}
