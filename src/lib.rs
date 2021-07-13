use std::path;
use std::io;
use std::fs;
use std::collections::HashMap;

pub enum CompressError {
    IoError(io::Error),
    InternalError(&'static str)
}

impl From<io::Error> for CompressError {
    fn from(err: io::Error) -> CompressError {
        CompressError::IoError(err)
    }
}


// FIXME: Deal with dictionary overflow
pub fn compress_file(in_file: &path::Path, out_file: &path::Path, code_size: usize) -> Result<(), CompressError> {

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
        add_string_to_dictionary(Vec::from(&buffer[..]), &mut dictionary, code_size)?;

        let new_byte = buffer.pop().unwrap(); // Earlier checks make this impossible to be 'None'.
        for byte in get_code_from_dictionary(&buffer, &dictionary, code_size)? {
            output.push(byte);
        }
        output.push(new_byte);
        buffer.clear();
    }

    if buffer.len() > 0 {
        for byte in get_code_from_dictionary(&buffer, &dictionary, code_size)? {
            output.push(byte);
        }
    }

    fs::write(out_file, &output[..])?;

    Ok(())
}

fn add_string_to_dictionary(string: Vec<u8>, dictionary: &mut HashMap<Vec<u8>, Vec<u8>>, code_size: usize) -> Result<(), CompressError> {
    let next_code = dictionary.len() + 256;
    let next_code = code_to_bytes(next_code, code_size)?;
    dictionary.insert(string, next_code);
    Ok(())
}

fn get_code_from_dictionary(string: &Vec<u8>, dictionary: &HashMap<Vec<u8>, Vec<u8>>, code_size: usize) -> Result<Vec<u8>, CompressError> {
    match string.len() {
        0 => {
            return Err(CompressError::InternalError("Cannot query dictionary with zero length string"));
        }
        1 => {
            let mut res = vec![0; code_size];
            res[code_size-1] = string[0];
            return Ok(res);
        }
        _ => {
            match dictionary.get(string) {
                Some(code) => Ok(code.clone()),
                None => Err(CompressError::InternalError("No code available for string"))
            }
        }
    }
}

fn code_to_bytes(code: usize, width: usize) -> Result<Vec<u8>, CompressError> {

    let mut res = vec![];
    let mut code = code;

    while code > 0 {
        let next_byte = (code & 0xFF) as u8;
        res.push(next_byte);
        code = code >> 8;
    }
    if res.len() > width {
        return Err(CompressError::InternalError("Code is larger than allowed width"));
    }
    while res.len() < width {
        res.push(0);
    }

    Ok(res)
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
