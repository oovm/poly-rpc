use std::{borrow::Borrow, marker::PhantomData};

use bincode::{
    config::standard,
    serde::{decode_from_slice, encode_to_vec},
};
use serde::{de::DeserializeOwned, Serialize};
use sled::{IVec, Tree};

use crate::{DiskMapError, Result};

mod iter;

/// A map store on hard disk
///
/// # Arguments
///
/// * `K`: key must can represent as `&[u8]`
/// * `V`: value must implement `Serialize` + `Deserialize`
///
/// # Examples
///
/// ```
/// use disk_map::DiskMap;
/// ```
pub struct DiskMap<K, V> {
    inner: Tree,
    typed: PhantomData<(K, V)>,
}

impl<K, V> Drop for DiskMap<K, V> {
    fn drop(&mut self) {
        self.inner.flush().ok();
    }
}

impl<K, V> From<Tree> for DiskMap<K, V> {
    fn from(value: Tree) -> Self {
        Self { inner: value, typed: Default::default() }
    }
}

impl<K, V> DiskMap<K, V>
where
    K: AsRef<[u8]>,
    V: Serialize + DeserializeOwned,
{
    /// Check if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Get the value by key name, return `None` if no such key
    #[inline]
    pub fn get<Q>(&self, key: Q) -> Option<V>
    where
        Q: Borrow<K>,
    {
        self.try_get(key.borrow().as_ref()).ok()
    }
    /// Raw api of `DiskMap::get`
    pub fn try_get(&self, key: &[u8]) -> Result<V> {
        match self.inner.get(key)? {
            Some(iv) => cast_iv(iv),
            None => Err(DiskMapError::KeyNotFound),
        }
    }
    /// Insert the value by key name, return `None` if no such key
    #[inline]
    pub fn insert<Q>(&self, key: Q, value: V) -> Option<V>
    where
        K: From<Q>,
    {
        self.try_insert(K::from(key).as_ref(), value).ok()
    }
    /// Raw api of `DiskMap::insert`
    pub fn try_insert(&self, key: &[u8], value: V) -> Result<V> {
        let v = encode_to_vec(&value, standard())?;
        match self.inner.insert(key, v.clone())? {
            Some(iv) => cast_iv(iv),
            None => Err(DiskMapError::KeyNotFound),
        }
    }
}

fn cast_iv<T>(s: IVec) -> Result<T>
where
    T: DeserializeOwned,
{
    Ok(decode_from_slice(s.as_ref(), standard())?.0)
}
