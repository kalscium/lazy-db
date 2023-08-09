use super::*;
use std::io::{Read, Write, BufReader, BufWriter, Error};
use std::fs::File;

pub enum FileWrapper {
    Reader(BufReader<File>),
    Writer(BufWriter<File>),
}

impl FileWrapper {
    /// Constructs a new `FileWrapper::Reader` varient
    pub fn new_reader(file: File) -> Self {
        Self::Reader(
            BufReader::new(file),
        )
    }

    /// Constructs a new `FileWrapper::Writer` varient
    pub fn new_writer(file: File) -> Self {
        Self::Writer(
            BufWriter::new(file),
        )
    }

    /// Writes a byte slice into the file
    pub fn write(&mut self, byte: &[u8]) -> Result<(), LDBError> {
        let writer = if let Self::Writer(w) = self { w }
            else { panic!("You cannot write on a reader") }; // Change later to use better error handling
        unwrap_result!(writer.write(byte) => |e| Err(LDBError::IOError(e)));
        Ok(())
    }

    /// Reads a set amount of bytes from a file by padding out undefined portions with 0u8
    pub fn read(&mut self, length: usize) -> Result<Box<[u8]>, LDBError> {
        let reader = if let Self::Reader(r) = self { r }
            else { panic!("You cannot read on a writer") }; // Change later to use better error handling
        let mut buffer = vec![0u8; length].into_boxed_slice();
        unwrap_result!(reader.read(&mut buffer) => |e| Err(LDBError::IOError(e)));
        Ok(buffer)
    }

    /// Deconstruct the wrapper properly with all of the buffers and such
    pub fn finish(self) -> Result<(), Error> {
        match self {
            Self::Reader(_) => (),
            Self::Writer(mut w) => w.flush()?,
        };
        Ok(())
    }

    /// Reads to the end of the file (consumes wrapper)
    pub fn read_to_end(self) -> Result<Box<[u8]>, LDBError> {
        let mut reader = if let Self::Reader(r) = self { r }
            else { panic!("You cannot read on a writer") }; // Change later to use better error handling
        let mut buffer = Vec::new();
        unwrap_result!(reader.read_to_end(&mut buffer) => |e| Err(LDBError::IOError(e)));
        Ok(buffer.into_boxed_slice())
    }
}