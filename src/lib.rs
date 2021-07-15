use std::path;
use std::fs;
use std::collections::HashMap;

pub mod errors;

use errors::{CompressError, DecompressError};

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

pub fn compress_file(in_file: &path::Path, out_file: &path::Path, code_size: CodeSize) -> Result<(), CompressError> {

    let mut buffer = vec![];
    let mut output = vec![];

    let mut dictionary = HashMap::new();
    let mut dictionary_locked = false;

    output.push(code_size.size() as u8); // Output size of code.

    for byte in fs::read(&in_file)? {
        buffer.push(byte);

        if get_code_from_dictionary(&buffer, &dictionary).is_some() {
            continue;
        }
        let mut will_lock_dictionary = false;
        if !dictionary_locked {
            will_lock_dictionary = add_string_to_dictionary(Vec::from(&buffer[..]), &mut dictionary, &code_size);
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


pub fn decompress_file(in_file: &path::Path, out_file: &path::Path) -> Result<(), DecompressError> {
    
    let mut input_iter = fs::read(in_file)?.into_iter();
    
    let code_size = input_iter.next().ok_or(DecompressError::MissingEmptyFileError)?;
    let code_size = match CodeSize::new(code_size as usize) {
        Ok(code_size) => code_size,
        Err(_) => return Err(DecompressError::CorruptInvalidFileError(1))
    };
    
    let mut code_buffer = vec![];
    let mut dictionary : Vec<_> = (0..256).map(|val| vec![val as u8; 1]).collect();
    let mut output = vec![];

    for byte in input_iter {
        if code_buffer.len() < code_size.size() {
            code_buffer.push(byte);
            if code_buffer.len() == code_size.size() {
                let code = vec_to_code(&code_buffer[..]);
                let string = dictionary.get(code).ok_or(DecompressError::CorruptInvalidFileError(2))?;
                for string_byte in string {
                    output.push(*string_byte);
                }
                if dictionary.len() == code_size.max() {
                    code_buffer.clear();
                }
            }
        } else {
            let code = vec_to_code(&code_buffer[..]);
            let mut new_string = dictionary.get(code).ok_or(DecompressError::CorruptInvalidFileError(3))?.clone();
            new_string.push(byte);
            output.push(byte);
            dictionary.push(new_string);
            code_buffer.clear();
        }
    }
    // TODO: What if the code_buffer isn't empty?

    fs::write(out_file, output)?;

    Ok(())
}

fn vec_to_code(vec: &[u8]) -> usize {
    let mut result = 0;

    for (idx, byte) in vec.iter().enumerate() {
        result = result | ((*byte as usize) << (idx * 8));
    }

    result
}


fn push_code_from_dictionary(string: &Vec<u8>, dictionary: &HashMap<Vec<u8>, usize>, code_size: &CodeSize, output: &mut Vec<u8>) -> Result<(), CompressError> {
    let code = get_code_from_dictionary(string, dictionary).ok_or("Failed to retreive code from dictionary")?;
    for byte in code_to_bytes(code, code_size)? {
        output.push(byte);
    }
    Ok(())
}

fn add_string_to_dictionary(string: Vec<u8>, dictionary: &mut HashMap<Vec<u8>, usize>, code_size: &CodeSize) -> bool {
    let next_code = dictionary.len() + 256;

    dictionary.insert(string, next_code);

    code_size.max() == next_code
}

fn get_code_from_dictionary(string: &Vec<u8>, dictionary: &HashMap<Vec<u8>, usize>) -> Option<usize> {
    match string.len() {
        0 => {
            None
        }
        1 => {
            Some(string[0] as usize)
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
