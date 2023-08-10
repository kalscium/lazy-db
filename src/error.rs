use std::{fmt, error::Error};
use crate::LazyType;

#[derive(Debug)]
pub enum LDBError {
    IOError(std::io::Error),
    WalkDirError(walkdir::Error),
    FileNotFound(String),
    InvalidLazyType(u8),
    IncorrectType(LazyType, String),
    InvalidUTF8String(Box<[u8]>),
    InvalidNumberByteLength(u8, String),
}

impl fmt::Display for LDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LDBError::*;
        match self {
            FileNotFound(p) => write!(f, "File '{p}' not found"),
            IOError(e) => write!(f, "IO Error: {:?}", e),
            WalkDirError(e) => write!(f, "WalkDir Error: {:?}", e),
            InvalidLazyType(t) => write!(f, "Invalid Lazy Type {t}"),
            IncorrectType(t1, t2) => write!(f, "Cannot read type '{0:?}' as '{1:?}'", t1, t2),
            InvalidUTF8String(x) => write!(f, "Bytes represent an invalid utf8 string: {:?}", x),
            InvalidNumberByteLength(x, t) => write!(f, "Invalid byte length '{x}' for number type '{t:?}'")
        }
    }
}

impl Error for LDBError {}