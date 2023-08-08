use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use super::*;

pub enum OFileMode {
    Read(BufReader<File>),
    Write(BufWriter<File>),
    Modify(BufReader<File>, BufWriter<File>),
}

impl OFileMode {
    #[inline]
    pub(super) fn new_read(file: File) -> OFileMode {
        OFileMode::Read(BufReader::new(file))
    }

    #[inline]
    pub(super) fn new_write(file: File) -> OFileMode {
        OFileMode::Write(BufWriter::new(file))
    }

    pub(super) fn modify_from_read(read_file_path: &Path, idx: &u64) -> Result<OFileMode, OFileError> {
        let read_file = unwrap_result!(File::open(read_file_path) => |e| Err(OFileError::IOError(e)));
        let mut reader = BufReader::new(read_file);
        let mut writer = BufWriter::new( unwrap_result!(File::create(read_file_path.with_extension("new")) => |e| Err(OFileError::IOError(e))) );
        let mut past = Option::<u8>::None;
        
        for _ in 0..*idx {
            let mut read = [0u8];
            let bytes_read = unwrap_result!(reader.read(&mut read) => |e| Err(OFileError::IOError(e)));
            if let Some(b) = past {
                unwrap_result!(writer.write(&[b]) => |e| Err(OFileError::IOError(e)));
            }; if bytes_read == 1 { past = Some(read[0]) };
        }
        
        Ok(OFileMode::Modify(reader, writer))
    }
}