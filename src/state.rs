use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Error, Note};

#[derive(Deserialize, Serialize)]
pub struct State {
    pub last_used_note: Option<String>,
    pub notes: HashMap<String, Note>,
}

impl State {
    pub fn new(last_used_note: Option<&str>, notes: HashMap<String, Note>) -> Self {
        Self {
            last_used_note: last_used_note.map(str::to_string),
            notes,
        }
    }

    pub fn load_from_file(file_path: &str) -> Result<Self, Error> {
        let file_contents =
            std::fs::read_to_string(file_path).or(Err(Error::StateFileNotFound {
                file_path: file_path.to_string(),
            }))?;

        toml::from_str(&file_contents).map_err(|e| Error::StateDeserialization {
            file_path: file_path.to_string(),
            serde_error: e,
        })
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), Error> {
        let serialized_note = toml::to_string(&self).map_err(Into::<Error>::into)?;
        std::fs::write(file_path, serialized_note).map_err(|e| Error::FailedToSaveState {
            file_path: file_path.to_string(),
            io_error: e,
        })
    }
}
