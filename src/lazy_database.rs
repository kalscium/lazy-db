use crate::*;
use std::path::{Path, PathBuf};
use std::fs;

#[macro_export]
macro_rules! search_database {
    (($ldb:expr) /$($($con:ident)?$(($can:expr))?)/ *) => {(|| {
        let database = $ldb;
        let container = database.as_container()?;
        $(
            $(let container = container.read_container(stringify!($con))?;)?
            $(let container = container.read_container($can)?;)?
        )*
        let result: Result<LazyContainer, LDBError> = Ok(container);
        result
    })()};

    (($ldb:expr) /$($($con:ident)?$(($can:expr))?)/ *::$($item:ident)?$(($obj:expr))?) => {(|| {
        let container = search_database!(($ldb) /$($($con)?$(($can))?)/ *)?;
        $(let result: Result<LazyData, LDBError> = container.read_data(stringify!($item));)?
        $(let result: Result<LazyData, LDBError> = container.read_data($obj);)?
        result
    })()};

    (($ldb:expr) $($item:ident)?$(($obj:expr))?) => {(|| {
        let database = $ldb;
        let container = database.as_container()?;
        $(let result: Result<LazyData, LDBError> = container.read_data(stringify!($item));)?
        $(let result: Result<LazyData, LDBError> = container.read_data($obj);)?
        result
    })()};
}

#[macro_export]
macro_rules! write_database {
    (($ldb:expr) $($item:ident)?$(($obj:expr))? = $func:ident($value:expr)) => {(|| {
        let database = $ldb;
        let container = database.as_container()?;
        $(LazyData::$func(container.data_writer(stringify!($item))?, $value)?;)?
        $(LazyData::$func(container.data_writer($obj)?, $value)?;)?
        Result::<(), LDBError>::Ok(())
    })()};

    (($ldb:expr) /$($($con:ident)?$(($can:expr))?)/ *::$($item:ident)?$(($obj:expr))? = $func:ident($value:expr)) => {(|| {
        let database = $ldb;
        let mut container = database.as_container()?;
        $({
            let con = $(stringify!($con))?$($can)?;
            container = match container.read_container(con) {
                Ok(x) => x,
                Err(LDBError::DirNotFound(_)) => container.new_container(con)?,
                Err(e) => return Err(e),
            }
        };)*

        LazyData::$func(container.data_writer($(stringify!($item))?)?$($obj)?, $value)?;
        Result::<(), LDBError>::Ok(())
    })()}
}

pub struct LazyDB {
    path: PathBuf,
    compressed: bool,
}

impl LazyDB {
    /// Initialises a new LazyDB directory at a specified path.
    /// 
    /// It will create the path if it doesn't already exist and initialise a metadata file with the current version of `lazy-db` if one doesn't exist already.
    /// 
    /// **WARNING:** if you initialise the database this way, you cannot compile it in future without errors being thrown!
    /// If you want to compile it, then use `LazyDB::init_db` instead.
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
            compressed: false,
        })
    }

    /// Initialise a new compiled `LazyDB` (compressed tarball) at the specified path.
    ///
    /// It will create the path if it doesn't already exist and initialise a metadata file with the current version of `lazy-db` if one doesn't exist already.
    pub fn init_db(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let dir_path = path.as_ref().with_extension("modb");
        let mut this = Self::init(dir_path)?;
        this.compressed = true;
        Ok(this)
    }

    /// Loads a pre-existing LazyDB directory at a specified path.
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
            compressed: false,
        })
    }

    /// Loads a pre-existing LazyDB file (compressed tarball) at a specified path
    /// 
    /// Loads LazyDB as `read-write` allowing for modification of the data within it.
    /// 
    /// If a directory version of the LazyDatabase exists, it will load the directory version instead of decompiling.
    /// 
    /// If the LazyDB is invalid, it will return an error.
    pub fn load_db(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let path = path.as_ref();

        { // Checks if other loaded version exists
            let dir_path = path.with_extension("modb");
            if dir_path.is_dir() { return Self::load_dir(dir_path) }
        }

        // Decompiles database
        let path = Self::decompile(path)?;
        let mut ldb = Self::load_dir(path)?;
        ldb.compressed = true;

        Ok(ldb)
    }

    #[inline]
    pub fn as_container(&self) -> Result<LazyContainer, LDBError> {
        LazyContainer::load(&self.path)
    }

    /// Compiles a modifiable `LazyDatabase` directory into a compressed tarball (doesn't delete the modifable directory).
    pub fn compile(&self) -> Result<PathBuf, std::io::Error> {
        use lazy_archive::*; // imports
        let tar = self.path.with_extension("tmp.tar");
        let new = self.path.with_extension("ldb");

        // Build and compress tarball
        build_tar(&self.path, &tar)?; // build tar
        compress_file(&tar, &new)?;

        // Clean-up
        fs::remove_file(tar)?;

        Ok(new)
    }

    /// Decompiles a compressed tarball `LazyDatabase` into a modifiable directory (doesn't remove the compressed tarball)
    pub fn decompile(path: impl AsRef<Path>) -> Result<PathBuf, LDBError> {
        use lazy_archive::*; // imports
        let path = path.as_ref();

        // Checks if the path exists
        if !path.is_file() { return Err(LDBError::FileNotFound(path.to_path_buf())) };

        // Decompress and unpack
        let tar = path.with_extension("tmp.tar");
        let unpacked = path.with_extension("modb");
        unwrap_result!(decompress_file(path, &tar) => |e| Err(LDBError::IOError(e)));
        unwrap_result!(unpack_tar(&tar, &unpacked) => |e| Err(LDBError::IOError(e)));

        // Clean-up
        unwrap_result!(fs::remove_file(tar) => |e| Err(LDBError::IOError(e)));
        
        Ok(unpacked)
    }
}

impl Drop for LazyDB {
    fn drop(&mut self) {
        if !self.compressed { return }; // If not compressed do nothing
        let ok = self.compile().is_ok();
        if !ok { return }; // Don't delete if not ok
        let _ = fs::remove_dir_all(&self.path);
    }
}