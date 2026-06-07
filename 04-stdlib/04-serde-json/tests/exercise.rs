// DO NOT EDIT — implement the solution in src/lib.rs

use serde_json_lesson::{active_users, from_json, to_json, User};

#[test]
fn round_trip_json() {
    let user = User {
        id: 1,
        name: "Ada".into(),
        active: true,
    };
    let json = to_json(&user).unwrap();
    let back = from_json(&json).unwrap();
    assert_eq!(user, back);
}

#[test]
fn from_json_parses() {
    let raw = r#"{"id":2,"name":"Grace","active":false}"#;
    let user = from_json(raw).unwrap();
    assert_eq!(user.id, 2);
    assert_eq!(user.name, "Grace");
    assert!(!user.active);
}

#[test]
fn active_users_filters() {
    let users = vec![
        User {
            id: 1,
            name: "a".into(),
            active: true,
        },
        User {
            id: 2,
            name: "b".into(),
            active: false,
        },
    ];
    let active = active_users(&users);
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].id, 1);
}
