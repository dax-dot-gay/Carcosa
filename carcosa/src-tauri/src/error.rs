use getset::CloneGetters;
use serde::{Deserialize, Serialize};
use strum::EnumProperty;

#[derive(thiserror::Error, Debug, strum::EnumProperty)]
pub enum Error {
    #[error("Unknown error: {0:?}")]
    #[strum(props(code = "sys.unknown"))]
    Unknown(#[from] anyhow::Error),

    #[error("I/O Error: {0:?}")]
    #[strum(props(code = "sys.io"))]
    Io(#[from] std::io::Error),

    #[error("Database Error: {0:?}")]
    #[strum(props(code = "sys.database"))]
    Database(#[from] redb::Error),

    #[error("Invalid table kind: {0} (expected 'unique' or 'multimap')")]
    #[strum(props(code = "validation.table_kind"))]
    TableKind(String),

    #[error("Validation failure for input {input}: {reason}")]
    #[strum(props(code = "validation.validation"))]
    Validation { input: String, reason: String },

    #[error("Base64 decoding error: {0:?}")]
    #[strum(props(code = "base64.decode"))]
    Base64Decode(#[from] base64::DecodeError),

    #[error("Failed to parse network identity: {0:?}")]
    #[strum(props(code = "net.id_parse"))]
    NetIdParse(#[from] iroh::KeyParsingError),

    #[error("JSON encoding/decoding error: {0:?}")]
    #[strum(props(code = "validation.json"))]
    JsonDecode(#[from] serde_json::error::Error),

    #[error("Tauri framework error: {0:?}")]
    #[strum(props(code = "sys.tauri"))]
    Tauri(#[from] tauri::Error)
}

macro_rules! db_errs {
    ($err:ident, $($errs:ident),+) => {
        impl From<redb::$err> for Error {
            fn from(value: redb::$err) -> Error {
                Error::Database(redb::Error::from(value))
            }
        }

        db_errs!($($errs),+);
    };
    ($err:ident) => {
        impl From<redb::$err> for Error {
            fn from(value: redb::$err) -> Error {
                Error::Database(redb::Error::from(value))
            }
        }
    }
}

db_errs!(
    TableError,
    CommitError,
    StorageError,
    DatabaseError,
    SavepointError,
    CompactionError,
    TransactionError,
    SetDurabilityError
);

impl Error {
    pub fn validation(input: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::Validation {
            input: input.into(),
            reason: reason.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct MetaError {
    code: String,
    message: String,
}

impl<T: Into<Error>> From<T> for MetaError {
    fn from(value: T) -> Self {
        let err = value.into();
        Self {
            code: err.get_str("code").unwrap_or("unknown").to_string(),
            message: err.to_string(),
        }
    }
}

impl MetaError {
    pub fn operation(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: format!("operation.{}", code.into()),
            message: message.into(),
        }
    }
}

pub type MetaResult<T> = std::result::Result<T, MetaError>;
