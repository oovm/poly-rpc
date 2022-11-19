use crate::PolyError;
use std::num::ParseIntError;

impl From<ParseIntError> for PolyError {
    fn from(value: ParseIntError) -> Self {
        PolyError::syntax_error(value.to_string()).with_source(value)
    }
}
