use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Note {
    /// Path to the file that contains the content of the note
    content_file_path: String,
    /// When the note was first created
    creation_date: DateTime<Local>,
    /// A short description describing the note.
    description: Option<String>,
}

impl Note {
    pub fn new(
        content_file_path: &str,
        creation_date: DateTime<Local>,
        description: Option<&str>,
    ) -> Self {
        Self {
            content_file_path: content_file_path.to_string(),
            creation_date,
            description: description.map(str::to_string),
        }
    }

    pub fn load_from_file(file_path: &str) -> Result<Self, Error> {
        let file_contents =
            std::fs::read_to_string(file_path).or(Err(Error::NoteFileNotFound {
                file_path: file_path.to_string(),
            }))?;

        toml::from_str(&file_contents).map_err(Into::into)
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), Error> {
        let serialized_note = toml::to_string(&self).map_err(Into::<Error>::into)?;
        std::fs::write(file_path, serialized_note).map_err(|e| Error::FailedToSaveNote {
            file_path: file_path.to_string(),
            io_error: e,
        })
    }
}
