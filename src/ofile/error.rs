use std::fmt;

impl<'a> std::error::Error for OFileError<'a> {}
#[derive(Debug)]
pub enum OFileError<'a> {
    // CannotWriteToFile(String),
    CannotReadFile(&'a str),
    IOError(std::io::Error),
    FileNotFound(String),
    EndOfStream,
}

impl<'a> fmt::Display for OFileError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use OFileError::*;
        match self {
            // CannotWriteToFile(p) => write!(f, "Cannot write to file '{p}' while in read-only mode"),
            CannotReadFile(p) => write!(f, "Cannot read file '{p}' while in write-only mode"),
            IOError(e) => write!(f, "IO Error: {e}"),
            FileNotFound(p) => write!(f, "Expected file '{p}' not found"),
            EndOfStream => write!(f, "End of file stream reached"),
        }
    }
}