pub mod ofile;
pub mod utils;
pub mod error;
pub mod lazy_type;

// Prelude
pub use crate::{
    error::{LDBErrContext, LDBError, LDBHandler},
    lazy_type::*,
};

#[macro_export]
macro_rules! const_eval {
    (($type:ty) $code:expr) => {{
        const RESULT: $type = $code;
        RESULT
    }};
}