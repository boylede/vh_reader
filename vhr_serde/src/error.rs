use std::fmt::{Display, Formatter};

use serde::de::Unexpected;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    WontImplement,
    NotYetImplemented,
    ReachedUnexpectedEnd,
    UnconsumedData,
    OverlargeData,
    NonAsciiString,
    Other,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        use Error::*;
        write!(
            f,
            "{}",
            match self {
                WontImplement => "We won't implement this feature",
                NotYetImplemented => "We haven't implemented this yet",
                ReachedUnexpectedEnd => "We reached the end of the input unexpectedly",
                UnconsumedData => "There was unexpected data at the end of the input",
                OverlargeData => "The data was larger than the format allowed",
                NonAsciiString => "An input string contained non-ascii characters",

                Other => {
                    "This error message has not been written yet"
                }
            }
        )
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        Error::Other
    }
}

impl serde::de::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        Error::Other
    }
}

impl serde::ser::StdError for Error {}

pub type Result<T> = std::result::Result<T, Error>;
