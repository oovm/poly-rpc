use sled::Error;

use crate::{DiskMapError, DiskMapError::CustomError};

impl From<Error> for DiskMapError {
    fn from(error: Error) -> Self {
        match error {
            Error::Io(e) => Self::IOError(e),
            _ => CustomError(error.to_string()),
        }
    }
}
