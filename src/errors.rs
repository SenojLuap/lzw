use std::io;

pub enum CompressError {
    IoError(io::Error),
    InternalError(&'static str),
}

impl From<io::Error> for CompressError {
    fn from(err: io::Error) -> CompressError {
        CompressError::IoError(err)
    }
}

impl From<&'static str> for CompressError {
    fn from(msg: &'static str) -> CompressError {
        CompressError::InternalError(msg)
    }
}

pub enum DecompressError {
    IoError(io::Error),
    InternalError(&'static str),
    MissingEmptyFileError,
    CorruptInvalidFileError(u16)
}

impl From<io::Error> for DecompressError {
    fn from(err: io::Error) -> DecompressError {
        DecompressError::IoError(err)
    }
}

impl From<&'static str> for DecompressError {
    fn from(msg: &'static str) -> DecompressError {
        DecompressError::InternalError(msg)
    }
}
