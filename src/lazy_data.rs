use std::path::{Path, PathBuf};
use std::io::Read;
use crate::*;

pub struct LazyData {
    pub path: PathBuf,
    pub lazy_type: LazyType,
    ofile: OFile,
}

impl LazyData {
    /// Creates a new `LazyData` file with the type of `LazyType::Void`
    #[inline]
    pub fn new_void(path: impl AsRef<Path>) -> Result<(), LDBError> { Self::uninit(path)?; Ok(()) }

    /// Creates an uninitialised `LazyData` file with the default type of `LazyType::Void`
    fn uninit(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let path = path.as_ref();
        let ofile = OFile::new(path);
        let ofile = match ofile {
            Err(e) => if let OFileError::IOError(e) = e {
                return Err(LDBError::IOError(e));
            } else { panic!("Shouldn't panic as OFile creation doesn't throw non IOError errors") }
            Ok(x) => x,
        };
        Ok(Self {
            path: path.to_path_buf(),
            lazy_type: LazyType::Void,
            ofile,
        })
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let path = path.as_ref();

        // Check for the existance of the path and if it's a file
        if !path.is_file() { return Err(LDBError::FileNotFound(path.to_string_lossy().to_string())) };

        // Initialise an OFile
        let mut ofile = match OFile::new(path) {
            Ok(o) => o,
            Err(e) =>
                if let OFileError::IOError(e) = e { return Err(LDBError::IOError(e)) }
                else { panic!("Creating an ofile shouldn't throw any other errors!") }
        };

        // read type
        // keeps idx at zero so avoid others from reading file type again
        let mut lazy_type = [0u8; 2];
        let reader = if let OFileMode::Read(r) = &mut ofile.mode { r } else { panic!("Shouldn't panic as ofile hasn't been modified till this point") };
        unwrap_result!(reader.read(&mut lazy_type) => |e| Err(LDBError::IOError(e)));

        Ok(Self {
            path: path.to_path_buf(),
            lazy_type: LazyType::from_bytes(lazy_type)?,
            ofile,
        })
    }

    /// ### **Lazy Action**
    /// ( Only returns internal ofile field of `Lazy Data` )
    /// 
    /// ---
    /// Collects the `LazyData` as a Lazy `OFile`.
    /// 
    /// Returns `LDBError::IncorrectType` if the LazyData type is not `LazyType::Binary`
    pub fn collect_ofile(self) -> Result<OFile, LDBError> {
        if self.lazy_type != LazyType::Binary { return Err(LDBError::IncorrectType(self.lazy_type, LazyType::Binary)) }
        Ok(self.ofile)
    }
}