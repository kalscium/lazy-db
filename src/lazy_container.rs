use crate::*;
use std::path::{Path, PathBuf};
use std::fs;

/// Used for reading from a `LazyContainer` with less boiler-plate
#[macro_export]
macro_rules! write_container {
    (($container:expr) $($item:ident)?$(($obj:expr))? = $func:ident($value:expr)) => {(|| {
        let container = &$container;
        $(LazyData::$func(container.data_writer(stringify!($item))?, $value)?;)?
        $(LazyData::$func(container.data_writer($obj)?, $value)?;)?
        Result::<(), LDBError>::Ok(())
    })()};

    (($container:expr) /$($($con:ident)?$(($can:expr))?)/ *::$($item:ident)?$(($obj:expr))? = $func:ident($value:expr)) => {(|| {
        let mut container = &mut $container;
        $({
            let con = $(stringify!($con))?$($can)?;
            container = match container.read_container(con) {
                Ok(x) => x,
                Err(LDBError::DirNotFound(_)) => container.new_container(con)?,
                Err(e) => return Err(e),
            }
        };)*

        $(LazyData::$func(container.data_writer(stringify!($item))?, $value)?;)?
        $(LazyData::$func(container.data_writer($obj)?, $value)?;)?
        Result::<(), LDBError>::Ok(())
    })()}
}

/// Used for reading from a `LazyContainer` with less boiler-plate
#[macro_export]
macro_rules! search_container {
    (($container:expr) /$($($con:ident)?$(($can:expr))?)/ *) => {(|| {
        let container = &$container;
        $(
            $(let container = container.read_container(stringify!($con))?;)?
            $(let container = container.read_container($can)?;)?
        )*
        let result: Result<LazyContainer, LDBError> = Ok(container);
        result
    })()};

    (($container:expr) /$($($con:ident)?$(($can:expr))?)/ *::$($item:ident)?$(($obj:expr))?) => {(|| {
        let container = search_container!(($container) /$($($con)?$(($can))?)/ *)?;
        $(let result: Result<LazyData, LDBError> = container.read_data(stringify!($item));)?
        $(let result: Result<LazyData, LDBError> = container.read_data($obj);)?
        result
    })()};

    (($container:expr) $($item:ident)?$(($obj:expr))?) => {(|| {
        let container = &$container;
        $(let result: Result<LazyData, LDBError> = container.read_data(stringify!($item));)?
        $(let result: Result<LazyData, LDBError> = container.read_data($obj);)?
        result
    })()};
}

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
        let file = unwrap_result!((fs::File::create(path)) err => LDBError::IOError(err));
        Ok(FileWrapper::new_writer(file))
    }

    /// Generates a nested `LazyContainer` within this container
    /// 
    /// If container already exists it will load it instead
    pub fn new_container(&self, key: impl AsRef<Path>) -> Result<LazyContainer, LDBError> {
        let path = self.path.join(&key);
        if path.is_dir() { return self.read_container(key) }; // If exists load instead
        Ok(unwrap_result!((LazyContainer::init(path)) err => LDBError::IOError(err)))
    }

    /// Reads nested `LazyData` within this container
    pub fn read_data(&self, key: impl AsRef<Path>) -> Result<LazyData, LDBError> {
        let path = self.path.join(key);
        if !path.is_file() { return Err(LDBError::FileNotFound(path)) };
        LazyData::load(path)
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