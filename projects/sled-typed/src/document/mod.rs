use std::{marker::PhantomData, path::Path};

use serde::{de::DeserializeOwned, Serialize};
use serde_binary::{binary_stream::Endian, from_slice, to_vec};
use sled::{Config, Db, IVec, Tree};

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
    database: Tree,
    typing: PhantomData<(K, V)>,
}

impl<K, V> Drop for DiskMap<K, V> {
    fn drop(&mut self) {
        self.database.flush().ok();
    }
}

impl<K, V> From<Tree> for DiskMap<K, V> {
    fn from(value: Tree) -> Self {
        Self { database: value, typing: Default::default() }
    }
}

impl<K, V> DiskMap<K, V>
where
    K: AsRef<[u8]>,
    V: Serialize + DeserializeOwned,
{
    /// Check if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.database.is_empty()
    }

    /// Get the value by key name, return `None` if no such key
    #[inline]
    pub fn get(&self, key: K) -> Option<V> {
        self.try_get(key).ok()
    }
    pub fn try_get(&self, key: K) -> Result<V> {
        let k = key.as_ref();
        match self.database.get(k)? {
            Some(iv) => cast_iv(iv),
            None => Err(DiskMapError::KeyNotFound),
        }
    }
    /// Insert the value by key name, return `None` if no such key
    #[inline]
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.try_insert(key, value).ok()
    }
    /// Trying to insert the value by key name, return `None` if no such key
    pub fn try_insert(&self, key: K, value: V) -> Result<V> {
        let k = key.as_ref();
        let v = to_vec(&value, Endian::Little)?;
        match self.database.insert(k, v.clone())? {
            Some(iv) => cast_iv(iv),
            None => Err(DiskMapError::KeyNotFound),
        }
    }
}

fn cast_iv<T>(s: IVec) -> Result<T>
where
    T: DeserializeOwned,
{
    Ok(from_slice(s.as_ref(), Endian::Little)?)
}
