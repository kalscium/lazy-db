use std::{fmt, error::Error, path::PathBuf};
use crate::LazyType;

#[derive(Debug)]
pub enum LDBError {
    IOError(std::io::Error),
    FileNotFound(PathBuf),
    DirNotFound(PathBuf),
    InvalidLazyType(u8),
    IncorrectType(LazyType, String),
    InvalidUTF8String(Box<[u8]>),
    InvalidNumberByteLength(u8, String),
    InvalidMetaVersion(PathBuf),
    IncompatibleVersion(crate::version::Version),
}

impl fmt::Display for LDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LDBError::*;
        match self {
            FileNotFound(p) => write!(f, "File '{}' not found", p.to_string_lossy()),
            DirNotFound(p) => write!(f, "Directory '{}' not found", p.to_string_lossy()),
            IOError(e) => write!(f, "IO Error: {:?}", e),
            InvalidLazyType(t) => write!(f, "Invalid Lazy Type {t}"),
            IncorrectType(t1, t2) => write!(f, "Cannot read type '{0:?}' as '{1:?}'", t1, t2),
            InvalidUTF8String(x) => write!(f, "Bytes represent an invalid utf8 string: {:?}", x),
            InvalidNumberByteLength(x, t) => write!(f, "Invalid byte length '{x}' for number type '{t:?}'"),
            InvalidMetaVersion(p) => write!(f, "Invalid version for `lazy-db` at '{}'", p.to_string_lossy()),
            IncompatibleVersion(v) => write!(f, "Found version '{v}' incompatible with current version '{}'", crate::VERSION),
        }
    }
}

impl Error for LDBError {}