use crate::DiskMapError;

impl From<std::io::Error> for DiskMapError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}
