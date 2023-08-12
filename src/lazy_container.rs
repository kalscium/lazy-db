use crate::*;
use std::path::{Path, PathBuf};
use std::fs;

/// A wrapper for a directory that holds individual `LazyData` files
pub struct LazyContainer {
    path: PathBuf,
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
        })
    }

    /// Loads a pre-existing `LazyContainer` directory at a specified path.
    /// 
    /// Will throw an error if the directory doesn't exist or there is an `io::Error`.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let path = path.as_ref().to_path_buf();

        // Checks if path exists or not
        if !path.is_dir() { return Err(LDBError::DirNotFound(path)) };

        // Constructs self
        Ok(Self {
            path,
        })
    }

    /// Generates a `FileWrapper` in write mode from a key (like a relative file path)
    /// 
    /// If the data already exists, it will try to remove it
    pub fn data_writer(&self, key: impl AsRef<Path>) -> Result<FileWrapper, LDBError> {
        let path = self.path.join(key);
        if path.is_file() { let _ = fs::remove_file(&path); }; // if files exists try remove it
        let file = unwrap_result!(fs::File::create(path) => |e| Err(LDBError::IOError(e)));
        Ok(FileWrapper::new_writer(file))
    }

    /// Generates a nested `LazyContainer` within this container
    /// 
    /// If container already exists it will load it instead
    pub fn new_container(&self, key: impl AsRef<Path>) -> Result<LazyContainer, LDBError> {
        let path = self.path.join(&key);
        if path.is_dir() { return self.read_container(key) }; // If exists load instead
        Ok(unwrap_result!(LazyContainer::init(path) => |e| Err(LDBError::IOError(e))))
    }

    /// Reads nested `LazyData` within this container
    pub fn read_data(&self, key: impl AsRef<Path>) -> Result<LazyData, LDBError> {
        let path = self.path.join(key);
        if !path.is_file() { return Err(LDBError::FileNotFound(path)) };
        Ok(LazyData::load(path)?)
    }

    /// Reads nexted `LazyContainer` within this container
    pub fn read_container(&self, key: impl AsRef<Path>) -> Result<LazyContainer, LDBError> {
        let path = self.path.join(key);
        if !path.is_dir() { return Err(LDBError::DirNotFound(path)) };
        LazyContainer::load(path)
    }

    /// Tries to remove item at specified key, if it fails nothing happens
    pub fn remove(&self, key: impl AsRef<Path>) {
        let path = self.path.join(key);
        if path.is_dir() {
            let _ = fs::remove_dir_all(path);
        } else {
            let _ = fs::remove_file(path);
        }
    }
}