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
                LazyData::new_binary(
                    FileWrapper::new_writer(
                        unwrap_result!(fs::File::create(meta) => |e| Err(LDBError::IOError(e)))
                    ), &[VERSION.major, VERSION.minor, VERSION.build],
                )?;
            }
        };

        // Construct Self
        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    /// Loads an pre-existing LazyDB directory at a specified path.
    /// 
    /// Loads LazyDB as `read-write` allowing for modification of the data within it.
    /// 
    /// If the LazyDB is invalid, it will return an error.
    pub fn load_dir(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let path = path.as_ref();

        // Checks if path exists
        if !path.is_dir() { return Err(LDBError::DirNotFound(path.to_path_buf())) };

        // Checks if `.meta` file exists or not
        let meta = path.join(".meta");
        if !meta.is_file() { return Err(LDBError::FileNotFound(meta)) };

        // Checks validity of version
        let read_version = LazyData::load(&meta)?.collect_binary()?;
        if read_version.len() != 3 { return Err(LDBError::InvalidMetaVersion(meta)) };
        let read_version = version::Version::new(read_version[0], read_version[1], read_version[2]);
        if !VERSION.is_compatible(&read_version) { return Err(LDBError::IncompatibleVersion(read_version)) };

        // Constructs Self
        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    #[inline]
    pub fn as_container(&self) -> Result<LazyContainer, LDBError> {
        LazyContainer::load(&self.path)
    }
}