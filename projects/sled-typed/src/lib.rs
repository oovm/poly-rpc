#[doc = include_str!("../Readme.md")]
#[forbid(missing_docs)]
pub use self::database::DiskMap;
pub use self::errors::DiskMapError;
pub(crate) use self::errors::Result;

mod database;
mod errors;
