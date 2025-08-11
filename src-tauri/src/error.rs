use std::sync::Arc;

use serde::{ Deserialize, Serialize };
use serde_json::Value;
use specta::Type;

use crate::templates::ValueType;

#[derive(Debug, Serialize, thiserror::Error, Clone)]
#[serde(into = "SerializableError")]
pub enum Error {
    #[error("Unhandled exception: {0:?}")] Unhandled(Arc<anyhow::Error>),
    #[error("Filesystem IO error: {0:?}")] Io(Arc<std::io::Error>),
    #[error("Zip extraction error: {0:?}")] Zip(Arc<zip::result::ZipError>),
    #[error("Selected project folder ({0}) already contains files.")] NonEmptyProjectFolder(String),
    #[error("Selected project path exists & is not a directory: {0}")] ExpectedProjectDirectory(
        String,
    ),
    #[error("JSON encoding error: {0:?}")] JsonEncoding(Arc<serde_json::Error>),
    #[error("Database backend error: {0:?}")] DatabaseError(Arc<native_db::db_type::Error>),
    #[error("No project is currently active!")] NoActiveProject,
    #[error("Invalid project selected at path {0}")] InvalidProjectSelection(String),
    #[error("Corrupted project at path {0}: {1}")] CorruptedProject(String, String),
    #[error("Internal tauri error: {0:?}")] TauriError(Arc<tauri::Error>),
    #[error("Invalid casting attempt: attempted to cast {value_type:?} as type {expected_type}")] InvalidCast {
        value_type: ValueType,
        expected_type: String
    },
    #[error("Failed to cast provided datatype: expected {value_type:?} but got {value:?}")] InvalidCastDatatype {
        value_type: ValueType,
        value: Value
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::Unhandled(Arc::new(value))
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(Arc::new(value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonEncoding(Arc::new(value))
    }
}

impl From<native_db::db_type::Error> for Error {
    fn from(value: native_db::db_type::Error) -> Self {
        Self::DatabaseError(Arc::new(value))
    }
}

impl From<tauri::Error> for Error {
    fn from(value: tauri::Error) -> Self {
        Self::TauriError(Arc::new(value))
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(value: zip::result::ZipError) -> Self {
        Self::Zip(Arc::new(value))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(tag = "err", content = "context", rename_all = "snake_case")]
pub enum SerializableError {
    Unhandled(String),
    Io(String),
    Zip(String),
    NonEmptyProjectFolder(String),
    ExpectedProjectDirectory(String),
    JsonEncoding(String),
    DatabaseError(String),
    NoActiveProject,
    InvalidProjectSelection(String),
    CorruptedProject(String),
    TauriError(String),
    InvalidCast {
        value_type: ValueType,
        expected_type: String
    },
    InvalidCastDatatype {
        value_type: ValueType,
        value: Value
    }
}

impl<T: Into<Error>> From<T> for SerializableError {
    fn from(value: T) -> Self {
        match value.into() {
            Error::Unhandled(error) => Self::Unhandled(error.to_string()),
            Error::Io(error) => Self::Io(error.to_string()),
            Error::Zip(error) => Self::Zip(error.to_string()),
            Error::NonEmptyProjectFolder(folder) => Self::NonEmptyProjectFolder(folder),
            Error::ExpectedProjectDirectory(path) => Self::ExpectedProjectDirectory(path),
            Error::JsonEncoding(error) => Self::JsonEncoding(error.to_string()),
            Error::DatabaseError(error) => Self::DatabaseError(error.to_string()),
            Error::NoActiveProject => Self::NoActiveProject,
            Error::InvalidProjectSelection(path) => Self::InvalidProjectSelection(path),
            Error::CorruptedProject(path, reason) =>
                        Self::CorruptedProject(
                            format!("The project at path {path} is corrupted: {reason}")
                        ),
            Error::TauriError(error) => Self::TauriError(error.to_string()),
            Error::InvalidCast { value_type, expected_type } => Self::InvalidCast { value_type, expected_type },
            Error::InvalidCastDatatype { value_type, value } => Self::InvalidCastDatatype { value_type, value }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
