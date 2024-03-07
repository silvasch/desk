use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Note {
    /// Path to the file that contains the content of the note
    pub content_file_path: String,
    /// When the note was first created
    pub creation_date: DateTime<Local>,
    /// A short description describing the note.
    pub description: Option<String>,
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
}
