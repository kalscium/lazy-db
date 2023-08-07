use std::path::{Path, PathBuf};
use std::io::Read;
use crate::*;

pub struct LazyData {
    pub path: PathBuf,
    pub lazy_type: LazyType,
    pub ofile: OFile,
}

impl LazyData {
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
        let mut lazy_type = [0u8; 2];
        let reader = if let OFileMode::Read(r) = &mut ofile.mode { r } else { panic!("Shouldn't panic as ofile hasn't been modified till this point") };
        unwrap_result!(reader.read(&mut lazy_type) => |e| Err(LDBError::IOError(e)));

        Ok(Self {
            path: path.to_path_buf(),
            lazy_type: LazyType::from_bytes(lazy_type)?,
            ofile,
        })
    }
}