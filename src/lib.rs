use std::collections::HashMap;

use clap::Parser;

mod app;
pub(crate) use app::App;

pub(crate) mod args;

mod error;
pub use error::Error;

mod note;
pub(crate) use note::Note;

mod state;
pub(crate) use state::State;

pub fn run() -> Result<(), Error> {
    let args = args::Args::parse();
    let xdg_dirs = xdg::BaseDirectories::with_prefix("desk").map_err(Into::<Error>::into)?;

    let state_file_path =
        xdg_dirs
            .place_data_file("state.toml")
            .or(Err(Error::StateFileNotFound {
                file_path: xdg_dirs
                    .get_data_file("state.toml")
                    .to_string_lossy()
                    .to_string(),
            }))?;
    let state_file_path = state_file_path.to_str().unwrap();

    let app = App::new(
        State::load_from_file(state_file_path).unwrap_or(State::new(None, HashMap::new())),
    );

    if args.list {
        let last_used_note = &app.state().last_used_note;

        for note_name in app.list_notes() {
            let note = app.get_note(note_name)?;

            let mut out = note_name.clone();

            if let Some(description) = &note.description {
                out.push_str(" - ");
                out.push_str(description);
            }

            if Some(note_name) == last_used_note.as_ref() {
                out.push('*');
            }

            println!("{}", out);
        }

        return Ok(());
    }

    app.state().save_to_file(state_file_path)?;

    Ok(())
}
