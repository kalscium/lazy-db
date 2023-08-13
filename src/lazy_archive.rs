use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use tar::Builder;
use lz4_flex::frame::{FrameEncoder, FrameDecoder};
use std::fs;

const BUFFER_SIZE: usize = 8192;

pub fn build_tar(path: impl AsRef<Path>, tar_path: impl AsRef<Path>) -> Result<(), io::Error> {
    let tar = File::create(tar_path)?;
    let mut builder = Builder::new(tar);

    recursive_tar_append(&mut builder, path, PathBuf::new())?;

    builder.finish()?;
    Ok(())
}

fn recursive_tar_append(builder: &mut Builder<File>, path: impl AsRef<Path>, tar_path: PathBuf) -> Result<(), io::Error> {
    for entry in std::fs::read_dir(path)?.into_iter().filter_map(|x| x.ok()) {
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            builder.append_file(
                tar_path.join(entry.file_name()),
                &mut File::open(&path)?
            )?;
        } else if file_type.is_dir() {
            recursive_tar_append(builder, path, tar_path.join(entry.file_name()))?;
        }
    };
    
    Ok(())
}

pub fn unpack_tar(path: impl AsRef<Path>, dir_path: impl AsRef<Path>) -> Result<(), io::Error> {
    let tar = File::open(path)?;
    let mut archive = tar::Archive::new(tar);
    let dir_path = dir_path.as_ref();
    fs::create_dir_all(dir_path)?;

    for entry in archive.entries()? {
        let mut entry = entry?;
        entry.unpack_in(dir_path)?;
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