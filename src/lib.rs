pub mod utils;
pub mod error;
pub mod lazy_type;
pub mod lazy_data;

// Prelude
pub use crate::{
    error::{LDBErrContext, LDBError, LDBHandler},
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