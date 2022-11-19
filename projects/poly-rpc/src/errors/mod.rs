use hyper::Method;

pub type PolyResult<T = ()> = Result<T, PolyError>;

#[derive(Debug, Copy, Clone)]
pub struct PolyError {
    kind: Box<PolyErrorKind>,
}

#[derive(Debug, Copy, Clone)]
pub enum PolyErrorKind {
    UnknownMethod { method: Method, path: String },
    CONTENT_TYPE,
}

impl PolyError {
    pub fn unknown_method(method: &Method) {}
}
