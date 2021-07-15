use std::env;
use std::process;
use std::path::Path;

use lzw;
use lzw::errors::DecompressError;

pub fn main() {
    let mut args = env::args();

    args.next();

    let input_file = match args.next() {
        Some(in_file) => in_file,
        None => {
            eprintln!("Must provide input file name/location");
            process::exit(1)
        }
    };
    let input_file = Path::new(&input_file);

    let output_file = match args.next() {
        Some(out_file) => out_file,
        None => {
            eprintln!("Must provide output file name/location");
            process::exit(1)
        }
    };
    let output_file = Path::new(&output_file);

    if let Err(err) = lzw::decompress_file(input_file, output_file) {
        match err {
            DecompressError::IoError(msg) => eprintln!("IO Error: {}", msg),
            DecompressError::CorruptInvalidFileError(location) => eprintln!("File is corrupt or invalid (Internal error code: {})", location),
            DecompressError::MissingEmptyFileError => eprintln!("File is missing or empty"),
            DecompressError::InternalError(msg) => eprintln!("Internal error: {}", msg)
        }
        process::exit(1);
    }
}