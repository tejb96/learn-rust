use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AppError {
    #[error("not found")]
    NotFound,
    #[error("validation failed on field {field}: {code}")]
    Validation { field: String, code: String },
}

/// Looks up `id` in `store`. Empty id → Validation; missing → NotFound.
pub fn find(store: &HashMap<String, String>, id: &str) -> Result<String, AppError> {
    Ok(String::new())
}

/// Wraps an error with additional context message prepended.
pub fn wrap(err: AppError, msg: &str) -> String {
    String::new()
}
