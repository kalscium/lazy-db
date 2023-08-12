use crate::*;
use std::path::{Path, PathBuf};
use std::fs;

pub struct LazyContainer {
    path: PathBuf,
    containers: Vec<PathBuf>,
    data: Vec<PathBuf>,
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
            data: Vec::new(),
        })
    }
}