use serde_binary::Error;

use crate::DiskMapError;

impl From<Error> for DiskMapError {
    fn from(error: Error) -> Self {
        DiskMapError::CustomError(error.to_string())
    }
}
