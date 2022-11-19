use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    str::FromStr,
};

use serde::{
    __private::de::{Content, ContentDeserializer},
    de::{
        value::{MapDeserializer, SeqDeserializer},
        DeserializeSeed, MapAccess, Visitor,
    },
    Deserialize, Deserializer,
};

use crate::PolyResult;

mod der;

#[derive(Default)]
pub struct QueryBuilder<'de> {
    inner: BTreeMap<&'de str, Content<'de>>,
}

impl Debug for QueryBuilder<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<'de> QueryBuilder<'de> {
    pub fn get(&self, key: &str) -> Option<&Content<'de>> {
        self.inner.get(key)
    }
    pub fn insert(&mut self, key: &'de str, value: Content<'de>) {
        self.inner.insert(key, value);
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
    let mut q = QueryBuilder::default();
    q.insert("id", Content::Str("str"));
    q.insert("user", Content::U64(u64::from_str("1001")?));
    println!("{:#?}", q);
    println!("{:#?}", TestEnum::deserialize(q));
    Ok(())
}
