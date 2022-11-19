use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};

use http::Request;
use hyper::Body;
use serde::{
    __private::de::Content,
    de::{DeserializeSeed, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use serde_types::{OneOrMany, ParsableValue};

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
    pub fn from_uri(request: &'request Request<Body>, headers: &'static [&'static str]) -> Self {
        let mut out = Default::default();
        let queries = match request.uri().query() {
            Some(s) => s,
            None => return out,
        };
        for query in queries.split("&") {
            println!("{query}")
        }
        out
    }
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
    pub fn insert_path<T>(&mut self, key: &'request str, value: T)
    where
        T: Into<QuerySegment<&'request>>,
    {
        let mut out = value.into().unwrap();
        match out.len() {
            1 => unsafe { self.inner.insert(key, Content::Str(out.get_unchecked(0))) },
            _ => self.inner.insert(key, Content::Seq(out.into_iter().map(Content::Str).collect())),
        }
    }
    pub fn cast_to<T>(self) -> PolyResult<T>
    where
        T: Deserialize<'request>,
    {
        Ok(T::deserialize(self.inner)?)
    }
    pub fn insert_header(&mut self) {}
}

#[derive(Debug)]
pub enum QuerySegment<'i> {
    One(&'i str),
    Many(&'i [&'i str]),
}

impl<'i> From<&i str> for QuerySegment<'i> {
    fn from(value: &'i str) -> Self {
        Self::One(value)
    }
}

impl<'i> From<&i [&'i str]> for QuerySegment<'i> {
    fn from(value: &i [&'i str]) -> Self {
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
