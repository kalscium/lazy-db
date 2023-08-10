pub mod error;
pub mod lazy_type;
pub mod lazy_data;

// Prelude
pub use crate::{
    error::LDBError,
    lazy_type::*,
    lazy_data::*,
};

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