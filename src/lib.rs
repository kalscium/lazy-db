pub mod error;
pub mod lazy_type;
pub mod lazy_data;
pub mod version;
pub mod lazy_database;
pub mod lazy_container;
mod lazy_archive;

// Prelude
pub use crate::{
    error::LDBError,
    lazy_type::*,
    lazy_data::*,
    lazy_database::*,
    lazy_container::*,
};

pub const VERSION: version::Version = version::Version::new(0, 1, 0);

#[macro_export]
macro_rules! const_eval {
    (($type:ty) $code:expr) => {{
        const RESULT: $type = $code;
        RESULT
    }};
}

#[macro_export]
macro_rules! unwrap_result {
    ($result:expr => $wrapper:expr) => {{
        let result = $result;
        if let Err(e) = result {
            return $wrapper(e);
        } result.unwrap()
    }}
}