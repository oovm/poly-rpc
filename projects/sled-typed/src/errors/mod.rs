use std::{
    error::Error,
    fmt::{Display, Formatter},
};

mod for_bincode;
mod for_sled;
mod for_std;

#[derive(Debug)]
pub enum DiskMapError {
    IOError(std::io::Error),
    CustomError(String),
    KeyNotFound,
}

pub type Result<T = ()> = std::result::Result<T, DiskMapError>;

impl Display for DiskMapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskMapError::IOError(e) => {
                write!(f, "IO Error: {e}")
            }
            DiskMapError::CustomError(e) => f.write_str(e),
            DiskMapError::KeyNotFound => write!(f, "No such key"),
        }
    }
}

impl Error for DiskMapError {}
