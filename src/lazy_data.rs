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
    wrapper: FileWrapper,
}

impl LazyData {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let path = path.as_ref();

        // Check for the existance of the path and if it's a file
        if !path.is_file() { return Err(LDBError::FileNotFound(path.to_string_lossy().to_string())) };

        // Get the reader
        let mut reader =
            FileWrapper::new_reader(unwrap_result!(std::fs::File::open(&path) => |e| Err(LDBError::IOError(e))));

        // Reads the byte repr of it's `LazyType`
        let lazy_type =
            LazyType::from_bytes(reader.read(2)?.as_ref())?;

        Ok(Self {
            path: path.to_path_buf(),
            lazy_type,
            wrapper: reader,
        })
    }
}