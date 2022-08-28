use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct BlankError;

impl Display for BlankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self}"))
    }
}

impl Error for BlankError {}
