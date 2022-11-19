use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use crate::Method;

mod for_der;
mod for_hyper;
mod for_std;

pub type PolyResult<T = ()> = Result<T, PolyError>;

#[derive(Debug)]
pub struct PolyError {
    kind: Box<PolyErrorKind>,
    source: Option<Box<dyn Error>>,
}

impl Display for PolyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for PolyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.source.as_ref() {
            Some(s) => Some(&**s),
            None => None,
        }
    }
}

impl serde::de::Error for PolyError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self { kind: Box::new(PolyErrorKind::Custom { message: msg.to_string() }), source: None }
    }
}

#[derive(Debug, Clone)]
pub enum PolyErrorKind {
    SyntaxError { message: String },
    UnknownEndPoint { method: Method, path: String },
    UnsupportedContentType { content: String },
    Custom { message: String },
}

impl From<PolyErrorKind> for PolyError {
    fn from(value: PolyErrorKind) -> Self {
        Self { kind: Box::new(value), source: None }
    }
}

impl PolyError {
    pub fn with_source<E>(mut self, e: E) -> Self
    where
        E: Error + 'static,
    {
        self.source = Some(Box::new(e));
        self
    }
    pub fn syntax_error(message: impl Into<String>) -> Self {
        PolyErrorKind::SyntaxError { message: message.into() }.into()
    }
    pub fn unknown_end_point(method: &Method, url_path: impl Into<String>) -> Self {
        PolyErrorKind::UnknownEndPoint { method: method.clone(), path: url_path.into() }.into()
    }
    pub fn unknown_content_type(content_type: impl Into<String>) -> Self {
        PolyErrorKind::UnsupportedContentType { content: content_type.into() }.into()
    }
}
