//! # Throrgan
//! 
//! `throrgan` is a music compiling library written in Rust by Jack Dinsmore. It
//! reads `.thr` files which describe notes to be played by a collection of 
//! `throrgan` instruments, and compiles them into .wav files that can be used, 
//! for example, as a royalty-free soundtrack for video games.

use std::path::Path;
use std::{fs, io};

mod errors;
mod parse;
mod instrument;
mod generator;
mod output;

/// Compiles a file and generates a wave file
/// # Errors
/// - Returns a `FileAlreadyExists` error if `output_file` already exists
/// - Returns a `FileDoesNotExist` error if `input_file` does not exist
pub fn compile(input_file: &str, output_file: &str) 
-> errors::Result<()> {
    // Check if the output file exists
    /*if Path::new(output_file).exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, 
            format!("The file {} already exists", input_file)).into());
    }*/
    if output_file.len() < 4 || !output_file[output_file.len()-4..].eq(".wav") {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, 
            "You must specify a valid .wav file as output").into());
    }

    // Open and read the input file
    let contents: String = fs::read_to_string(input_file)?.parse()?;

    let header = parse::get_header(&contents, input_file)?;

    parse::generate(&header, &contents, input_file, output_file)?;

    Ok(())
}