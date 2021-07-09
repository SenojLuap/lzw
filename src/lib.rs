use std::path;
use std::io;
use std::fs;
use std::collections::HashMap;

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
    let mut output = vec![];

    let mut dictionary = HashMap::new();

    for byte in fs::read(&in_file)? {
        buffer.push(byte);
        if buffer.len() == 1 { // All single-byte sequences are assumed to be in the dictionary
            continue;
        }
        if dictionary.contains_key(&Vec::from(&buffer[..])) {
            continue;
        }
        dictionary.insert(Vec::from(&buffer[..]), (dictionary.len()+256) as u32);

        let new_byte = buffer.pop().unwrap(); // Earlier checks make this impossible to be 'None'.
        let old_code = if buffer.len() == 1 {
                buffer.pop().unwrap() as u32
            } else {
                dictionary.get(&buffer[..]).unwrap().clone()
            };
        for byte in old_code.to_be_bytes() {
            output.push(byte);
        }
        output.push(new_byte);
        buffer.clear();
    }

    fs::write(out_file, &output[..])?;

    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let left = vec![1, 2, 3];
        let right = vec![1, 2, 3];
        assert_eq!(left, right);
    }
}
