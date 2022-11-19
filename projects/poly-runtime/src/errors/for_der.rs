use crate::PolyError;
use serde::de::value::Error;

impl From<Error> for PolyError {
    fn from(value: Error) -> Self {
        panic!("{value}")
    }
}
