use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum CommonError {
    ConnDb(String),
    Unknown(String),
}
