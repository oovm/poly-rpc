pub use super::*;

impl Debug for Database {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl From<Db> for Database {
    fn from(value: Db) -> Self {
        Self { inner: value }
    }
}
