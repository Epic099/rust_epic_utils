use std::io;
use serde_json;

#[derive(Debug)]
pub enum FileError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl From<io::Error> for FileError {
    fn from(e: io::Error) -> Self { FileError::Io(e) }
}
impl From<serde_json::Error> for FileError {
    fn from(e: serde_json::Error) -> Self { FileError::Json(e) }
}