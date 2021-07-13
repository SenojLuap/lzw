use std::env;
use std::process;
use std::path::Path;
use lzw::{self, CompressError, CodeSize};

struct Config {
    file_name: String,
    output_file: String
}

pub fn main() {
    let config = match parse_args(env::args()) {
        Ok(config) => config,
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    };

    let compress_result = lzw::compress_file(Path::new(&config.file_name), Path::new(&config.output_file), CodeSize::new(2).unwrap());
    if let Err(err) = compress_result {
        match err {
            CompressError::IoError(io_error) => {
                eprintln!("IO Error: {}", io_error);
                process::exit(1);
            }
            CompressError::InternalError(msg) => {
                eprintln!("Internal error: {}", msg);
                process::exit(1);
            }
        }
    }
}


fn parse_args(mut args: env::Args) -> Result<Config, &'static str> {
    args.next();

    let file_name = match args.next() {
        Some(file_name) => file_name,
        None => return Err("Must provide name of file to compress")
    };

    let output_file = match args.next() {
        Some(output_file) => output_file,
        None => return Err("Must specify output file name/location")
    };

    Ok(Config {
        file_name: file_name,
        output_file: output_file
    })
}