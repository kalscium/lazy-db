use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use tar::Builder;
use lz4_flex::frame::{FrameEncoder, FrameDecoder};

const BUFFER_SIZE: usize = 8192;

pub fn build_tar(path: impl AsRef<Path>, tar_path: impl AsRef<Path>) -> Result<(), io::Error> {
    let tar = File::create(tar_path)?;
    let mut builder = Builder::new(tar);

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let mut file = File::open(path)?;
        builder.append_file(path, &mut file)?;
    };

    builder.finish()?;
    Ok(())
}

pub fn unpack_tar(path: impl AsRef<Path>, dir_path: impl AsRef<Path>) -> Result<(), io::Error> {
    let tar = File::open(path)?;
    let mut archive = tar::Archive::new(tar);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = dir_path.as_ref().join(entry.path()?);
        entry.unpack(path)?;
    };

    Ok(())
}

pub fn compress_file(path: impl AsRef<Path>, out_path: impl AsRef<Path>) -> Result<(), io::Error> {
    let mut file = File::open(path)?;
    let out = File::create(out_path)?;
    
    let mut encoder = FrameEncoder::new(out);
    let mut buffer = [0u8; BUFFER_SIZE]; // 8KB read buffer

    // Read data from input and write compressed output
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 { break };
        encoder.write_all(&buffer[..bytes_read])?;
    }

    encoder.finish()?;
    Ok(())
}

pub fn decompress_file(path: impl AsRef<Path>, out_path: impl AsRef<Path>) -> Result<(), io::Error> {
    let file = File::open(path)?;
    let mut out = File::create(out_path)?;
    let mut decoder = FrameDecoder::new(file);
    let mut buffer = [0u8; BUFFER_SIZE];

    // Read compressed data and write decompressed version
    loop {
        let bytes_read = decoder.read(&mut buffer)?;
        if bytes_read == 0 { break };
        out.write_all(&buffer[..bytes_read])?;
    };

    Ok(())
}