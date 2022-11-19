use crate::{PolyError, PolyResult};
use hyper::{
    header::{CONTENT_TYPE},
    Body, HeaderMap,
};

pub enum SupportContentType {
    Text,
    Json,
}

fn resolve_body(body: &Body, headers: &HeaderMap) -> PolyResult {
    let ct = match headers.get(CONTENT_TYPE) {
        Some(s) => s.to_str()?.to_ascii_lowercase(),
        None => return Err(PolyError::unknown_content_type("EMPTY")),
    };
    let ct = match ct.as_str() {
        s if s.starts_with("text/plain") => {
            SupportContentType::Text
        }
        _=> return Err(PolyError::unknown_content_type(ct))
    };
    match ct {
        SupportContentType::Text => {
            body.
        }
        SupportContentType::Json => {

        }
    }
}
