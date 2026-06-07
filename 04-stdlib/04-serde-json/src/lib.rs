use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub active: bool,
}

/// Serializes `user` to a JSON string.
pub fn to_json(user: &User) -> Result<String, serde_json::Error> {
    Ok(String::new())
}

/// Deserializes a JSON string into `User`.
pub fn from_json(raw: &str) -> Result<User, serde_json::Error> {
    Ok(User {
        id: 0,
        name: String::new(),
        active: false,
    })
}

/// Returns users whose `active` field is true.
pub fn active_users(users: &[User]) -> Vec<User> {
    vec![]
}
