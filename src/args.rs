use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// An optional name to operate on.
    /// When storing notes, a randomly generated name is used by default.
    /// When inspecting notes, the last created/referenced name is used by default.
    #[arg(short, long)]
    pub name: Option<String>,

    /// The type of the note.
    #[arg(short, long, default_value_t = NoteType::default())]
    pub ty: NoteType,

    /// Don't just print the value, but provide more information about it, like the origin and creation date.
    #[arg(short, long)]
    pub info: bool,

    /// A short description describing the note. If the note type is a file, by default the description is set to it's path.
    #[arg(long)]
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, Default, ValueEnum)]
pub enum NoteType {
    /// Raw text
    #[default]
    Raw,
    /// A file
    File,
}

impl std::fmt::Display for NoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NoteType::Raw => "raw",
                NoteType::File => "file",
            }
        )
    }
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
