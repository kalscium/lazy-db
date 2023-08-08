mod reading;
mod writing;
mod file_wrapper;

pub use reading::*;
pub use writing::*;
pub use file_wrapper::*;

use std::path::{Path, PathBuf};
use crate::*;

pub enum ActiveData {
    SignedNum(LazyINumType, i64),
    UnsignedNum(LazyINumType, u64),
    Float(LazyFloatType, f64),
}

pub struct LazyData {
    pub path: PathBuf,
    pub lazy_type: LazyType,
}

impl LazyData {
    /// Creates an uninitialised `LazyData` file with the default type of `LazyType::Void`
    fn uninit(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let path = path.as_ref();
        Ok(Self {
            path: path.to_path_buf(),
            lazy_type: LazyType::Void,
        })
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let path = path.as_ref();

        // Check for the existance of the path and if it's a file
        if !path.is_file() { return Err(LDBError::FileNotFound(path.to_string_lossy().to_string())) };

        // Initialise an OFile

        // Reads the byte repr of it's `LazyType`
        let mut lazy_type = [0u8; 2];
        // unwrap_result!(reader.read(&mut lazy_type) => |e| Err(LDBError::IOError(e)));

        Ok(Self {
            path: path.to_path_buf(),
            lazy_type: LazyType::from_bytes(lazy_type)?,
        })
    }
}