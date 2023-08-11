use crate::*;
use std::path::{Path, PathBuf};
use std::fs;

pub struct LazyDB {
    path: PathBuf,
}

impl LazyDB {
    /// Initialises a new LazyDB at a specified path.
    /// 
    /// It will create the path if it doesn't already exist and initialise a metadata file with the current version of `lazy-db` if one doesn't exist already.
    pub fn init(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let path = path.as_ref();

        // Check if path exists or not if init it
        if !path.is_dir() { unwrap_result!(fs::create_dir_all(path) => |e| Err(LDBError::IOError(e))) };
        
        { // Check if `.meta` file exists if not 
            let meta = path.join(".meta");
            if !meta.is_file() {
                // Write version
                LazyData::new_binary(meta, &[VERSION.major, VERSION.minor, VERSION.build])?;
            }
        };

        // Construct Self
        Ok(Self {
            path: path.to_path_buf(),
        })
    }
}