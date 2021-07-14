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

impl From<&'static str> for CompressError {
    fn from(msg: &'static str) -> CompressError {
        CompressError::InternalError(msg)
    }
}

pub enum CodeSize {
    Two,
    Three,
    Four
}

impl CodeSize {
    pub fn new(size: usize) -> Result<CodeSize, &'static str> {
        match size {
            2 => Ok(CodeSize::Two),
            3 => Ok(CodeSize::Three),
            4 => Ok(CodeSize::Four),
            _ => Err("Invalid code size")
        }
    }

    pub fn max(&self) -> usize {
        match self {
            CodeSize::Two => 65_535,
            CodeSize::Three => 16_777_215,
            CodeSize::Four => 4_294_967_295
        }
    }

    pub fn size(&self) -> usize {
        match self {
            CodeSize::Two => 2,
            CodeSize::Three => 3,
            CodeSize::Four => 4
        }
    }
}

// FIXME: Deal with dictionary overflow
pub fn compress_file(in_file: &path::Path, out_file: &path::Path, code_size: CodeSize) -> Result<(), CompressError> {

    let mut buffer = vec![];
    let mut output = vec![];

    let mut dictionary = HashMap::new();
    let mut dictionary_locked = false;

    for byte in fs::read(&in_file)? {
        buffer.push(byte);

        if get_code_from_dictionary(&buffer, &dictionary, &code_size).is_some() {
            continue;
        }

        let mut will_lock_dictionary = false;
        if !dictionary_locked {
            will_lock_dictionary = add_string_to_dictionary(Vec::from(&buffer[..]), &mut dictionary, &code_size)?;
        }

        let new_byte = buffer.pop().unwrap(); // Earlier checks make this impossible to be 'None'.
        push_code_from_dictionary(&buffer, &dictionary, &code_size, &mut output)?;

        if dictionary_locked {
            buffer.clear();
            buffer.push(new_byte);
        } else {
            output.push(new_byte);
            buffer.clear();
        }

        dictionary_locked = dictionary_locked || will_lock_dictionary;
    }

    if buffer.len() > 0 {
        push_code_from_dictionary(&buffer, &dictionary, &code_size, &mut output)?;
    }

    fs::write(out_file, &output[..])?;

    Ok(())
}

fn push_code_from_dictionary(string: &Vec<u8>, dictionary: &HashMap<Vec<u8>, Vec<u8>>, code_size: &CodeSize, output: &mut Vec<u8>) -> Result<(), &'static str> {
    let code = get_code_from_dictionary(string, dictionary, code_size).ok_or("Failed to retreive code from dictionary")?;
    for byte in code {
        output.push(byte);
    }
    Ok(())
}

fn add_string_to_dictionary(string: Vec<u8>, dictionary: &mut HashMap<Vec<u8>, Vec<u8>>, code_size: &CodeSize) -> Result<bool, CompressError> {
    let next_code = dictionary.len() + 256;

    let next_code_as_bytes = code_to_bytes(next_code, code_size)?;
    dictionary.insert(string, next_code_as_bytes);

    Ok(code_size.max() == next_code)
}

fn get_code_from_dictionary(string: &Vec<u8>, dictionary: &HashMap<Vec<u8>, Vec<u8>>, code_size: &CodeSize) -> Option<Vec<u8>> {
    match string.len() {
        0 => {
            None
        }
        1 => {
            let mut res = vec![0; code_size.size()];
            res[0] = string[0];
            return Some(res);
        }
        _ => {
            match dictionary.get(string) {
                Some(code) => Some(code.clone()),
                None => None
            }
        }
    }
}

fn code_to_bytes(code: usize, code_size: &CodeSize) -> Result<Vec<u8>, CompressError> {

    let mut res = vec![];
    let mut code = code;

    while code > 0 {
        let next_byte = (code & 0xFF) as u8;
        res.push(next_byte);
        code = code >> 8;
    }
    if res.len() > code_size.size() {
        return Err(CompressError::InternalError("Code is larger than allowed width"));
    }
    while res.len() < code_size.size() {
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
