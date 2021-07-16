use std::env;
use std::process;
use std::path::Path;
use std::io::Error;

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
        let (msg, exit_code) = match err {
            DecompressError::IoError(msg) => {
                (format!("IO Error: {}", msg), match Error::last_os_error().raw_os_error() {
                    Some(error_code) => error_code,
                    None => 1
                })
            }
            DecompressError::CorruptInvalidFileError(location) => {
                (format!("File is corrupt or invalid (Internal error code: {})", location), 13)
            }
            DecompressError::MissingEmptyFileError => {
                (format!("File is missing or empty"), 2)
            }
            DecompressError::InternalError(msg) => {
                (format!("Internal error: {}", msg), 1)
            }
        };
        eprintln!("{}", msg);
        process::exit(exit_code);
    }
}