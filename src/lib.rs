use clap::Parser;

pub(crate) mod args;

mod error;
pub use error::Error;

pub fn run() -> Result<(), Error> {
    let args = args::Args::parse();

    dbg!(args);

    Ok(())
}
