//! # Parse
//! 
//! This file processes all of the text files found in .thr files and turns
//! them into commands. It loads instruments, compiles wave files, and performs
//! other tasks.

use crate::generator;
use crate::instrument::Instrument;
use crate::errors::{Result, ParseError};

enum Mode {
    Instruments,
}

/// Stores all the information needed to know about a piece of music, including
/// what instruments it is written for
pub struct Header {
    instruments: Vec<Instrument>,
}

/// Makes and returns a header object for the text of a file
pub fn get_header(content: &str, name: &str) -> Result<Header> {
    let mut mode : Option<Mode> = None;
    let mut ret = Header { instruments:Vec::new() };

    for (num, line) in content.lines().enumerate() {
        if line.is_empty() {
            continue
        }

        if match line.chars().next() {
            Some(c) => c,
            None => '?'
        } == '#' {
            // Change the mode
            mode = match &line[1..] {
                "instruments" => Some(Mode::Instruments),
                _ => return Err(ParseError::InvalidMode(name.to_string(), num)
                .into()),
            };
        }

        else {
            match mode {
                None => return Err(
                    ParseError::NoModeDeclared(name.to_string(), num).into()),
                Some(ref m) => match m {
                    Instruments => {
                        let mut items = line.split_whitespace();
                        let instrument_name = match items.next() {
                            None => return Err(ParseError::Unknown(
                                name.to_string(), num).into()),
                            Some(name) => name,
                        };
                        let vol = match items.next() {
                            None => return Err(ParseError::Unknown(
                                name.to_string(), num).into()),
                            Some(v) => v,
                        }.parse::<f32>()?;
                        ret.instruments.push(
                            Instrument::new(instrument_name, vol)?);
                        // Continue implementation
                        // Then document the rest.`
                    }
                }
            }
        }
    }
    Ok(ret)
}