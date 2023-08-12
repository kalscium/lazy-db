use crate::*;
use std::path::{Path, PathBuf};
use std::fs;
use std::ffi::OsString;

pub struct LazyContainer {
    path: PathBuf,
    containers: Vec<OsString>,
    files: Vec<OsString>,
}

impl LazyContainer {
    /// Initialises a new, empty `LazyContainer` at the specified path.
    pub fn init(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let path = path.as_ref();

        // Checks if path exists or not
        if !path.is_dir() { fs::create_dir_all(path)? };
        
        // Constructs self
        Ok(Self {
            path: path.to_path_buf(),
            containers: Vec::new(),
            files: Vec::new(),
        })
    }

    /// Loads a pre-existing `LazyContainer` directory at a specified path.
    /// 
    /// Will throw an error if the directory doesn't exist or there is an `io::Error`.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let path = path.as_ref().to_path_buf();

        // Checks if path exists or not
        if !path.is_dir() { return Err(LDBError::DirNotFound(path)) };

        // Values
        let mut containers = Vec::<OsString>::new();
        let mut files = Vec::<OsString>::new();

        // Loads files and directories
        let dir = unwrap_result!(fs::read_dir(&path) => |e| Err(LDBError::IOError(e)));
        for entry in dir.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    containers.push(entry.file_name());
                } else { files.push(entry.file_name()) }
            }
        };

        // Constructs self
        Ok(Self {
            path,
            containers,
            files,
        })
    }
}