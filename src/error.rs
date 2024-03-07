use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidTypeArgument {
        given_type: String,
    },

    NoteFileNotFound {
        file_path: String,
    },
    FailedToSaveNote {
        file_path: String,
        io_error: std::io::Error,
    },
    Deserialization {
        serde_error: toml::de::Error,
    },
    Serialization {
        serde_error: toml::ser::Error,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        write!(
            f,
            "{}",
            match self {
                InvalidTypeArgument { given_type } =>
                    format!("'{}' is not a valid type", given_type),

                NoteFileNotFound { file_path } => format!(
                    "tried to load a note from a file that does not exist: '{}'",
                    file_path
                ),
                FailedToSaveNote { file_path, io_error } => format!("failed to save the note to '{}': {}", file_path, io_error.to_string().to_lowercase()),
                Deserialization { serde_error } => format!(
                    "tried to load a note from a file that does not contain valid toml: {}",
                    serde_error
                ),
                Serialization { serde_error } => format!("failed to store the note on disk, because the data could not be converted to toml: {}", serde_error)
            }
        )
    }
}

impl std::error::Error for Error {}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self::Deserialization { serde_error: value }
    }
}

impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Self::Serialization { serde_error: value }
    }
}
