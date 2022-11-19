use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
};

use serde::{__private::de::Content, de::Visitor, Deserialize, Deserializer};

#[derive(Default)]
pub struct QueryBuilder<'de> {
    inner: BTreeMap<&'de str, Content<'de>>,
}

impl Debug for QueryBuilder<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<'de> Deserializer<'de> for QueryBuilder<'de> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.inner.remove("text") {
            Some(Content::String(s)) => visitor.visit_string(s),
            Some(Content::Str(s)) => visitor.visit_str(s),
            _ => {
                panic!()
            }
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("{name:?}");
        println!("{fields:?}");
        for field in fields {
            match self.inner.remove(field) {
                Some(Content::Bool(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::U8(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::U16(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::U32(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::U64(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::I8(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::I16(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::I32(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::I64(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::F32(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::F64(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::Char(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::String(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::Str(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::ByteBuf(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::Bytes(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::Newtype(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::Seq(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::Map(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::Some(v)) => {
                    panic!("{v:?}")
                }
                Some(Content::None) => {}
                Some(Content::Unit) => {}
                None => {
                    panic!("{field}")
                }
            }
        }
        panic!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}
impl<'de> QueryBuilder<'de> {
    pub fn get(&self) {}
    pub fn insert_path(&mut self, key: &'static str, value: &'de str) {
        self.inner.insert(key, Content::Str(value));
    }
    pub fn insert_header(&mut self) {}
}

#[derive(Debug, Deserialize)]
pub struct Test {
    id: usize,
}

#[test]
fn test() {
    let q = QueryBuilder::default();
    println!("{:#?}", Test::deserialize(q))
}
