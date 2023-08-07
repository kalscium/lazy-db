pub mod error;
pub mod ofile_mode;
pub use error::*;
pub use ofile_mode::*;

use std::path::Path;
use crate::unwrap_result;
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::{BufReader, BufWriter, Read, Write};

pub struct OFile {
    pub file_path: PathBuf,
    pub mode: OFileMode,
    pub current: Option<u8>,
    pub idx: u64,
}

impl OFile {
    pub fn new<'a>(file_path: impl AsRef<Path>) -> Result<Self, OFileError<'a>> {
        if file_path.as_ref().exists() {
            Self::new_read(file_path)
        } else { Self::new_write(file_path) }
    }

    fn new_write<'a>(file_path: impl AsRef<Path>) -> Result<Self, OFileError<'a>> {
        let file = unwrap_result!(File::create(&file_path) => |e| Err(OFileError::IOError(e)));
        Ok(Self {
            file_path: file_path.as_ref().to_path_buf(),
            mode: OFileMode::new_write(file),
            idx: 0,
            current: None,
        })
    }

    fn new_read<'a>(file_path: impl AsRef<Path>) -> Result<Self, OFileError<'a>> {
        let file = unwrap_result!(File::open(&file_path) => |e| Err(OFileError::IOError(e)));
        Ok(Self {
            file_path: file_path.as_ref().to_path_buf(),
            mode: OFileMode::new_read(file),
            idx: 0,
            current: None,
        })
    }

    pub fn read(&mut self) -> Result<u8, OFileError<'_>> {
        let reader: &mut BufReader<File> = match &mut self.mode {
            OFileMode::Read(r) => r,
            OFileMode::Modify(r, _) => r,
            OFileMode::Write(_) => return Err(OFileError::CannotReadFile(self.file_path.to_str().unwrap()))
        };
        
        let mut bytes = [0u8];
        // Trys to read bytes, if reaches end of stream, throws erorr
        if unwrap_result!(reader.read(&mut bytes) => |e| Err(OFileError::IOError(e))) == 0usize {
            return Err(OFileError::EndOfStream);
        };


        self.idx += 1;
        self.current = Some(bytes[0]);
        
        Ok(bytes[0])
    }

    pub fn write(&mut self, value: u8) -> Result<(), OFileError<'_>> {
        let writer: &mut BufWriter<File> = match &mut self.mode {
            OFileMode::Write(w) => w,
            OFileMode::Modify(_, w) => w,
            OFileMode::Read(_) => {
                self.mode = OFileMode::modify_from_read(&self.file_path, &self.idx)?;
                if let OFileMode::Modify(_, w) = &mut self.mode {
                    w
                } else { panic!("shouldn't logically panic") }
            },
        };

        unwrap_result!(writer.write(&[value]) => |e| Err(OFileError::IOError(e)));
        Ok(())
    }

    pub fn skip(&mut self, amount: u64) -> Result<(), OFileError<'_>> {
        match &mut self.mode {
            OFileMode::Write(w) => {
                for _ in 0..amount {
                    unwrap_result!(w.write(&[0u8]) => |e| Err(OFileError::IOError(e)));
                }; Ok(())
            },
            OFileMode::Read(r) => {
                let mut byte = [0u8];
                for _ in 0..amount {
                    // Check if reached end of stream
                    if unwrap_result!(r.read(&mut byte) => |e| Err(OFileError::IOError(e))) == 0 {
                        return Err(OFileError::EndOfStream);
                    };
                }; self.current = Some(byte[0]);
                Ok(())
            },
            OFileMode::Modify(r, w) => {
                let mut byte = [0u8];
                for _ in 0..amount {
                    // Check if reached end of stream
                    if unwrap_result!(r.read(&mut byte) => |e| Err(OFileError::IOError(e))) == 0 {
                        return Err(OFileError::EndOfStream);
                    };
                    // Write read value to new file
                    unwrap_result!(w.write(&byte) => |e| Err(OFileError::IOError(e)));
                }; self.current = Some(byte[0]);
                Ok(())
            }
        }
    }

    #[inline]
    pub fn finish<'a>(mut self) -> Result<(), OFileError<'a>> {
        self.flush()?;
        std::mem::forget(self);
        Ok(())
    }

    fn flush<'a>(&mut self) -> Result<(), OFileError<'a>> {
        let mut rturn = Ok(());

        // Try to Flush Buffers
        // If error occurs, program will just continue though return it in the end
        match &mut self.mode {
            OFileMode::Read(_) => (),
            OFileMode::Write(w) => if let Err(e) = w.flush() {rturn = Err(OFileError::IOError(e))},
            OFileMode::Modify(_, w) => if let Err(e) = w.flush() {rturn = Err(OFileError::IOError(e))},
        };

        // if mode is modify, rename old file to `<file>.old`, new file to `<file>` and delete the old file
        if let OFileMode::Modify(_, _) = self.mode {
            let old_path = self.file_path.with_extension("old");
            unwrap_result!(fs::rename(&self.file_path, &old_path) => |e| Err(OFileError::IOError(e)));
            unwrap_result!(fs::rename(self.file_path.with_extension("new"), &self.file_path) => |e| Err(OFileError::IOError(e)));
            unwrap_result!(fs::remove_file(old_path) => |e| Err(OFileError::IOError(e)));
        }

        rturn
    }
}

impl Drop for OFile {
    #[inline]
    fn drop(&mut self) {
        // Ignore errors
        let _ = self.flush();
    }
}