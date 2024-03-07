use std::fmt;

#[derive(Debug)]
pub enum Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        write!(f, "{}", todo!())
    }
}

impl std::error::Error for Error {}
