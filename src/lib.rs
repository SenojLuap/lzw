use std::path;
use std::io;
use std::fs;

pub enum CompressError {
    IoError(io::Error),
}

impl From<io::Error> for CompressError {
    fn from(err: io::Error) -> CompressError {
        CompressError::IoError(err)
    }
}

pub fn compress_file(in_file: &path::Path, out_file: &path::Path) -> Result<(), CompressError> {

    let mut buffer = vec![];

    for byte in fs::read(&in_file)? {
        buffer.push(byte);
        if buffer.len() == 1 { // All single-byte sequences are assumed to be in the dictionary
            continue;
        }
        
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
