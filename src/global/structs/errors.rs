use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum Errors {
    EncryptionError,
    DecrpytionError,
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::EncryptionError => "cannot encrypt data, check if everything is valid",
            Self::DecrpytionError => "cannot decrypt data, check if the key is correct",
        })
    }
}

impl Error for Errors {}
