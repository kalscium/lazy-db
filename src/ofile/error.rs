use std::fmt;

impl std::error::Error for OFileError {}
#[derive(Debug)]
pub enum OFileError {
    // CannotWriteToFile(String),
    CannotReadFile(std::path::PathBuf),
    IOError(std::io::Error),
    FileNotFound(std::path::PathBuf),
    EndOfStream,
}

impl fmt::Display for OFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use OFileError::*;
        match self {
            // CannotWriteToFile(p) => write!(f, "Cannot write to file '{p}' while in read-only mode"),
            CannotReadFile(p) => write!(f, "Cannot read file '{}' while in write-only mode", p.to_string_lossy()),
            IOError(e) => write!(f, "IO Error: {e}"),
            FileNotFound(p) => write!(f, "Expected file '{}' not found", p.to_string_lossy()),
            EndOfStream => write!(f, "End of file stream reached"),
        }
    }
}