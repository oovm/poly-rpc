use bincode::error::{DecodeError, EncodeError};

use crate::DiskMapError;

impl From<EncodeError> for DiskMapError {
    fn from(error: EncodeError) -> Self {
        DiskMapError::CustomError(error.to_string())
    }
}

impl From<DecodeError> for DiskMapError {
    fn from(error: DecodeError) -> Self {
        DiskMapError::CustomError(error.to_string())
    }
}
