pub use super::*;

impl<K, V> From<Tree> for DiskMap<K, V> {
    fn from(value: Tree) -> Self {
        Self { inner: value, typed: Default::default() }
    }
}
