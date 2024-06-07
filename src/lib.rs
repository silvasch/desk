use std::collections::HashMap;

use chrono::Local;
use clap::Parser;

mod app;
pub(crate) use app::App;

pub(crate) mod args;

mod error;
pub use error::Error;

mod generate_name;
pub(crate) use generate_name::generate_name;

mod note;
pub(crate) use note::Note;

mod state;
pub(crate) use state::State;

pub fn run() -> Result<(), Error> {
    let args = args::Args::parse();
    let stdin = args::get_piped_stdin();

    let xdg_dirs =
        xdg::BaseDirectories::with_prefix(clap::crate_name!()).map_err(Into::<Error>::into)?;

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

    let notes_dir_path = format!("{}/notes", xdg_dirs.get_data_home().to_str().unwrap());

    let mut app = App::new(
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

    match stdin {
        Some(stdin) => {
            let name = args.name.unwrap_or(generate_name(app.list_notes()));

            let description = args.description;

            let note = Note::new(
                &format!("{}/{}", notes_dir_path, name),
                chrono::Local::now(),
                None,
                description.as_deref(),
            );

            xdg_dirs
                .place_data_file(format!("notes/{}.toml", name))
                .expect("this was already called once before");

            app.set_note(&name, note, &stdin, args.force)?;

            println!("Stored the data in the note '{}'", name);
        }
        None => {
            let name = match args.name.or(app.state().last_used_note.clone()) {
                Some(name) => name,
                None => return Ok(()),
            };

            let mut note = app.get_note(&name)?.clone();

            let content = note.read_content()?;

            if let Some(description) = args.description {
                note.description = Some(description);
            } else {
                println!("{}", content);
            }

            if args.info {
                println!("Accessing note '{}'", name);
                println!(
                    "Description: '{}'",
                    note.description.clone().unwrap_or_default()
                );
                println!(
                    "Created: {}",
                    note.creation_date.format("%Y-%m-%d %H:%M:%S")
                );
                println!(
                    "Last accessed: {}",
                    match note.last_accessed_date {
                        Some(last_accessed_date) =>
                            last_accessed_date.format("%Y-%m-%d %H:%M:%S").to_string(),
                        None => "never".to_string(),
                    }
                )
            }

            note.last_accessed_date = Some(Local::now());

            app.set_last_used_note(&name);
            app.set_note(&name, note, &content, true)?;
        }
    }

    app.state().save_to_file(state_file_path)?;

    Ok(())
}
