use crate::{Error, Note, State};

pub struct App {
    state: State,
}

impl App {
    pub fn new(state: State) -> Self {
        Self { state }
    }

    pub fn set_note(&mut self, name: &str, note: Note, force: bool) -> Result<(), Error> {
        if !force && self.state.notes.contains_key(name) {
            return Err(Error::NoteAlreadyExists {
                name: name.to_string(),
            });
        }

        self.state.notes.insert(name.to_string(), note);

        Ok(())
    }

    pub fn get_note(&self, name: &str) -> Result<&Note, Error> {
        self.state.notes.get(name).ok_or(Error::NoteNotFound {
            name: name.to_string(),
        })
    }

    pub fn list_notes(&self) -> Vec<&String> {
        self.state.notes.keys().collect()
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}
