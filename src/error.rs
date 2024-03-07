use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidTypeArgument {
        given_type: String,
    },

    NoteAlreadyExists {
        name: String,
    },
    NoteNotFound {
        name: String,
    },

    StateFileNotFound {
        file_path: String,
    },
    FailedToSaveState {
        file_path: String,
        io_error: std::io::Error,
    },
    StateDeserialization {
        file_path: String,
        serde_error: toml::de::Error,
    },
    StateSerialization {
        serde_error: toml::ser::Error,
    },

    XdgError {
        xdg_error: xdg::BaseDirectoriesError,
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

                NoteAlreadyExists { name } => format!(
                    "a note with the name '{}' already exists. use the '-f' option to overwrite it",
                    name
                ),
                NoteNotFound { name } => format!("could not find a note with the name '{}'", name),

                StateFileNotFound { file_path } => format!(
                    "tried to load the state from a file that does not exist: '{}'",
                    file_path
                ),
                FailedToSaveState {
                    file_path,
                    io_error,
                } => format!(
                    "failed to save the state to '{}': {}",
                    file_path,
                    io_error.to_string().to_lowercase()
                ),
                StateDeserialization {
                    file_path,
                    serde_error,
                } => format!(
                    "failed to deserialize the state file at '{}': {}",
                    file_path, serde_error
                ),
                StateSerialization { serde_error } =>
                    format!("failed to serialize the state: {}", serde_error),

                XdgError { xdg_error } => format!("xdg error: {}", xdg_error),
            }
        )
    }
}

impl std::error::Error for Error {}

impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Self::StateSerialization { serde_error: value }
    }
}

impl From<xdg::BaseDirectoriesError> for Error {
    fn from(value: xdg::BaseDirectoriesError) -> Self {
        Self::XdgError { xdg_error: value }
    }
}
