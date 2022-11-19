use super::*;

impl<'de, T> Deserialize<'de> for OneOrMany<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let content = Content::deserialize(deserializer)?;
        if let Ok(o) = T::deserialize(ContentRefDeserializer::<D::Error>::new(&content)) {
            return Ok(OneOrMany { inner: vec![o] });
        }
        if let Ok(o) = Vec::<T>::deserialize(ContentRefDeserializer::<D::Error>::new(&content)) {
            return Ok(OneOrMany { inner: o });
        }
        Err(Error::custom(format!("`{:?}` does not `{t}` or sequence of `{t}`", content.as_str(), t = type_name::<T>())))
    }
}

impl<T> From<T> for OneOrMany<T> {
    fn from(value: T) -> Self {
        OneOrMany::one(value)
    }
}

impl<T> From<Vec<T>> for OneOrMany<T> {
    fn from(value: Vec<T>) -> Self {
        Self { inner: value }
    }
}
impl<T> From<&[T]> for OneOrMany<T>
where
    T: Clone,
{
    fn from(value: &[T]) -> Self {
        Self { inner: value.to_vec() }
    }
}
