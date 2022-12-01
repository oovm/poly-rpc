use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};

use http::{HeaderMap, Request};
use hyper::Body;
use serde::{
    __private::de::Content,
    de::{DeserializeSeed, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use serde_types::ParsableValue;

use crate::PolyResult;

mod der;

#[derive(Default)]
pub struct QueryBuilder<'de> {
    pub inner: ParsableValue<'de>,
}

impl Debug for QueryBuilder<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<'request> QueryBuilder<'request> {
    pub fn from_headers(r: &'request Request<Body>, keys: &'request [&'request str]) -> Self {
        let mut out = QueryBuilder::default();
        let queries = match r.uri().query() {
            Some(s) => s,
            None => return out,
        };
        for query in queries.split("&") {
            println!("{query}")
        }
        for key in keys {
            out.add_header(r.headers(), key)
        }
        out
    }
    fn add_header(&mut self, headers: &'request HeaderMap, key: &'request str) {
        match headers.get(key) {
            Some(s) => match s.to_str() {
                Ok(o) => self.inner.insert(key, Content::Str(o)),
                Err(_) => {}
            },
            None => {}
        }
    }
    pub fn with_path<T>(mut self, key: &'request str, value: T) -> Self
    where
        T: Into<QuerySegment<'request>>,
    {
        let c = match value.into() {
            QuerySegment::One(v) => Content::Str(v),
            QuerySegment::Many(v) => Content::Seq(v.iter().map(|s| Content::Str(*s)).collect()),
        };
        self.inner.insert(key, c);
        self
    }
    pub fn cast_to<T>(self) -> PolyResult<T>
    where
        T: Deserialize<'request>,
    {
        Ok(T::deserialize(self.inner)?)
    }
    pub fn insert_header(&mut self) {}
}

impl<'request> QueryBuilder<'request> {
    pub fn uri_path(req: &'request Request<Body>, prefix: &'static str) -> PolyResult<Vec<&'request str>> {
        let mut out = vec![];
        let url = req.uri().path();
        println!("UIR: {}", url);
        if !url.starts_with(prefix) {
            return Err(todo!());
        }
        for name in req.uri().path().split('/') {
            out.push(name)
        }
        Ok(out)
    }
    pub fn text(s: &'request str) -> Self {
        Self { inner: ParsableValue::new(Content::Str(s)) }
    }
    pub fn get(&self, key: &str) -> Option<&Content<'request>> {
        self.inner.get(key)
    }
}

#[derive(Debug)]
pub enum QuerySegment<'i> {
    One(&'i str),
    Many(&'i [&'i str]),
}

impl<'i> From<&&'i str> for QuerySegment<'i> {
    fn from(value: &&'i str) -> Self {
        Self::One(value)
    }
}

impl<'i> From<&'i str> for QuerySegment<'i> {
    fn from(value: &'i str) -> Self {
        Self::One(value)
    }
}

impl<'i> From<&'i [&'i str]> for QuerySegment<'i> {
    fn from(value: &'i [&'i str]) -> Self {
        Self::Many(value)
    }
}

#[derive(Debug, Deserialize)]
pub struct Test {
    id: String,
    user: usize,
}

#[test]
fn test() -> PolyResult {
    let mut q = QueryBuilder::text("12");
    // q.insert("id", Content::Str("str"));
    // q.insert("user", Content::U64(u64::from_str("1001")?));
    println!("{:#?}", q);
    println!("{:#?}", u32::deserialize(q.inner));
    Ok(())
}
