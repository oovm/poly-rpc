pub use sled::Config;

#[doc = include_str!("../Readme.md")]
#[forbid(missing_docs)]
pub use self::document::DiskMap;
pub(crate) use self::errors::Result;
pub use self::{database::Database, errors::DiskMapError};

mod database;
mod document;
mod errors;
