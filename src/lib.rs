use clap::Parser;

mod app;
pub(crate) use app::App;

pub(crate) mod args;

mod error;
pub use error::Error;

mod note;
pub(crate) use note::Note;

pub fn run() -> Result<(), Error> {
    let args = args::Args::parse();

    dbg!(args);

    Ok(())
}
