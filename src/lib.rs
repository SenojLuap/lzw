use std::path;
use std::fs;
use std::collections::HashMap;

pub mod errors;

use errors::{CompressError};

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


pub fn decompress_file(in_file: &path::Path, out_file: &path::Path) -> Result<(), CompressError> {
    unimplemented!();
    /*
    let output = vec![];
    let in_buffer = fs::read(in_file)?;

    let code_size = match in_buffer.pop() {
        Some(byte) => byte,
        None => return Err(CompressError::MissingEmptyFileError)
    };
    let code_size = match CodeSize::new(code_size as usize) {
        Ok(size) => size,
        Err(_) => return Err(CompressError::CorruptInvalidFileError)
    };

    let dictionary = HashMap::new();
    let in_buffer = in_buffer.iter();

    let mut dictionary_locked = false;

    let mut next_code = get_code_from_input(&in_buffer, &code_size)?;

    while next_code.is_some() {
        if dictionary_locked {
            let next_string = get_string_from_dictionary(next_code.unwrap(), &dictionary, &code_size);
        } else {

        }
    }

    Ok(())
    */
}

fn get_string_from_dictionary<'a>(code: Vec<u8>, dictionary: &'a HashMap<Vec<u8>, Vec<u8>>, code_size: &CodeSize) -> Option<&'a Vec<u8>> {
    unimplemented!()
    /*
    if dictionary.contains_key(&code) {
        return dictionary.get(&code);
    }

    let mut 

    None*/
}


fn get_code_from_input<'a, T>(input: &'a T, code_size: &CodeSize) -> Result<Option<Vec<u8>>, CompressError> 
        where T: Iterator<Item = &'a u8> {
    unimplemented!();
    /*
    let result = vec![];

    for i in 0..code_size.size() {
        let next = match input.next() {
            Some(next_byte) => next_byte,
            None => {
                if result.is_empty() {
                    return Ok(None);
                }
                return Err(CompressError::CorruptInvalidFileError);
            }
        };
        result.push(next.clone());
    }

    Ok(Some(result))
    */
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
