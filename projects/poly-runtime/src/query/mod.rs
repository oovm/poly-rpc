use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};

use crate::PolyResult;
use serde::{
    __private::de::{Content, ContentDeserializer},
    de::{
        value::{MapDeserializer, SeqDeserializer},
        DeserializeSeed, MapAccess, Visitor,
    },
    Deserialize, Deserializer,
};
use serde_types::ParsableValue;

mod der;

pub struct QueryBuilder<'de> {
    inner: ParsableValue<'de>,
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
    pub fn text(s: &'de str) -> Self {
        Self { inner: Content::Str(s) }
    }
    pub fn get(&self, key: &str) -> Option<&Content<'de>> {
        match &self.inner {
            Content::Map(map) => {
                for (k, v) in map.iter().rev() {
                    if k.as_str()? == key {
                        return Some(v);
                    }
                }
            }
            _ => unreachable!(),
        }
        None
    }
    pub fn insert(&mut self, key: &'de str, value: Content<'de>) {
        match &mut self.inner {
            Content::Map(map) => {
                map.push((Content::Str(key), value));
            }
            _ => {}
        }
    }
    pub fn insert_header(&mut self) {}
}

#[derive(Debug, Deserialize)]
pub struct Test {
    id: String,
    user: usize,
}
#[derive(Debug, Deserialize)]
pub enum TestEnum {}

#[test]
fn test() -> PolyResult {
    let mut q = QueryBuilder::text("12");
    // q.insert("id", Content::Str("str"));
    // q.insert("user", Content::U64(u64::from_str("1001")?));
    println!("{:#?}", q);
    println!("{:#?}", u32::deserialize(q));
    Ok(())
}
