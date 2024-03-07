use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// An optional name to operate on.
    /// When storing data, a randomly generated name is used by default.
    /// When inspecting data, the last created/referenced name is used by default.
    #[arg(short, long)]
    pub name: Option<String>,

    /// The type of the data.
    #[arg(short, long)]
    pub ty: DataType,

    /// Don't just print the value, but provide more information about it, like the origin and creation date.
    #[arg(short, long)]
    pub info: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum DataType {
    /// Raw text
    Raw,
    /// A file
    File,
}

pub fn get_piped_stdin() -> Option<String> {
    if atty::is(atty::Stream::Stdin) {
        None
    } else {
        Some(
            std::io::stdin()
                .lines()
                .map(|v| v.unwrap())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
