use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Note {
    /// Path to the file that contains the content of the note
    pub content_file_path: String,
    /// When the note was first created
    pub creation_date: DateTime<Local>,
    /// When the note was last read
    pub last_accessed_date: Option<DateTime<Local>>,
    /// A short description describing the note.
    pub description: Option<String>,
}

impl Note {
    pub fn new(
        content_file_path: &str,
        creation_date: DateTime<Local>,
        last_accessed_date: Option<DateTime<Local>>,
        description: Option<&str>,
    ) -> Self {
        Self {
            content_file_path: content_file_path.to_string(),
            creation_date,
            last_accessed_date,
            description: description.map(str::to_string),
        }
    }

    pub fn read_content(&self) -> Result<String, Error> {
        std::fs::read_to_string(&self.content_file_path).map_err(|e| Error::NoteRead {
            file_path: self.content_file_path.clone(),
            io_error: e,
        })
    }

    pub fn write_content(&self, content: &str) -> Result<(), Error> {
        std::fs::write(&self.content_file_path, content).map_err(|e| Error::NoteWrite {
            file_path: self.content_file_path.clone(),
            io_error: e,
        })
    }
}
