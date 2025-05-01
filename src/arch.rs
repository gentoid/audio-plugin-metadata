use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryArch {
    X86,
    X86_64,
    Unknown,
}

#[derive(Debug, Error)]
pub enum ArchDetectError {
    #[error("File too small to be a valid PE executable")]
    FileTooSmall,
    #[error("Invalid MZ header")]
    InvalidMZHeader,
    #[error("Invalid PE signature")]
    InvalidPESignature,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn detect_binary_arch(path: &Path) -> Result<BinaryArch, ArchDetectError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut buffer = [0u8; 64];
    reader.read_exact(&mut buffer)?;

    if &buffer[0..2] != b"MZ" {
        return Err(ArchDetectError::InvalidMZHeader);
    }

    let pe_offset = u32::from_le_bytes(buffer[0x3C..0x40].try_into().unwrap());

    reader.seek(SeekFrom::Start(pe_offset as u64))?;
    reader.read_exact(&mut buffer[0..8])?;

    if &buffer[0..4] != b"PE\0\0" {
        return Err(ArchDetectError::InvalidPESignature);
    }

    let machine = u16::from_le_bytes(buffer[4..6].try_into().unwrap());

    Ok(match machine {
        0x014c => BinaryArch::X86,
        0x8664 => BinaryArch::X86_64,
        _ => BinaryArch::Unknown,
    })
}
