use crate::PolyError;
use hyper::header::ToStrError;

impl From<ToStrError> for PolyError {
    fn from(value: ToStrError) -> Self {
        panic!("{value}")
    }
}
