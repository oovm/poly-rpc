use std::fmt::{Debug, Formatter};

use serde::{
    __private::de::{Content, ContentDeserializer},
    de::{
        value::{MapDeserializer, SeqDeserializer},
        DeserializeSeed, MapAccess, Visitor,
    },
    Deserialize, Deserializer,
};
mod der;

pub struct QueryBuilder<'de> {
    inner: Content<'de>,
}

impl Default for QueryBuilder<'_> {
    fn default() -> Self {
        Self { inner: Content::Map(vec![]) }
    }
}

impl Debug for QueryBuilder<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<'de> QueryBuilder<'de> {
    pub fn get(&self) {}
    pub fn insert_usize(&mut self, key: &'static str, value: &'de str) {
        match &mut self.inner {
            Content::Map(v) => v.push((Content::Str(key), Content::Str(value))),
            _ => {}
        }
    }
    pub fn insert_header(&mut self) {}
}

#[derive(Debug, Deserialize)]
pub struct Test {
    user: usize,
}

#[test]
fn test() {
    let mut q = QueryBuilder::default();
    q.insert_usize("user", "1001");
    println!("{:#?}", q);
    println!("{:#?}", Test::deserialize(q))
}
