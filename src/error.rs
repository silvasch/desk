use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidTypeArgument(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        write!(
            f,
            "{}",
            match self {
                InvalidTypeArgument(type_argument) =>
                    format!("'{}' is not a valid type", type_argument),
            }
        )
    }
}

impl std::error::Error for Error {}
