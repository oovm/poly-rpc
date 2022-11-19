use super::*;
use serde::de::{MapAccess, Visitor};
use std::fmt::Formatter;

impl<'de> Visitor<'de> for HelloRequest {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        todo!()
    }

    fn visit_map<A>(mut self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        match map.next_key() {
            _ => {}
        }

        todo!()
    }
}
