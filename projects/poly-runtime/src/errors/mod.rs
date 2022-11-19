mod for_hyper;

use crate::Method;

pub type PolyResult<T = ()> = Result<T, PolyError>;

#[derive(Debug, Copy, Clone)]
pub struct PolyError {
    kind: Box<PolyErrorKind>,
}

#[derive(Debug, Copy, Clone)]
pub enum PolyErrorKind {
    UnknownEndPoint { method: Method, path: String },
    UnsupportedContentType { content: String },
}

impl PolyError {
    pub fn unknown_end_point(method: &Method, url_path: impl Into<String>) -> Self {
        let kind = PolyErrorKind::UnknownEndPoint { method: method.clone(), path: url_path.into() };
        Self { kind: Box::new(kind) }
    }
    pub fn unknown_content_type(content_type: impl Into<String>) -> Self {
        let kind = PolyErrorKind::UnsupportedContentType { content: content_type.into() };
        Self { kind: Box::new(kind) }
    }
}
