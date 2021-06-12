//! # Parse
//! 
//! This file processes all of the text files found in .thr files and turns
//! them into commands. It loads instruments, compiles wave files, and performs
//! other tasks.

use crate::generator::Breakdown;
use crate::instrument::Instrument;
use crate::errors::{Result, ParseError};

/// Stores parsing information about which part of the file we're in.
/// `Instruments` is the instrument declaring stage, `Signatures` is for other
/// things like tempo, and `Music` is the notes itself.
enum Mode {
    Instruments,
    Signature,
    Music
}

/// Stores all the information needed to know about a piece of music, including
/// what instruments it is written for
pub struct Header {
    instruments: Vec<Instrument>,
    tempo: i32,
    begin_music: usize,
    end_music: usize,
}

impl Header {
    /// Be able to generate an empty header to be loaded into
    fn empty() -> Header {
        Header { instruments:Vec::new(), tempo: 0, begin_music: 0,
            end_music: 0 }
    }

    /// Check that the header has been fully filled
    fn verify(self, name: &str) -> Result<Header> {
        if self.instruments.len() == 0 || self.tempo == 0 ||
            self.begin_music == 0 || self.end_music == 0 {
            return Err(ParseError::IncompleteHeader(name.to_string()).into());
        }
        Ok(self)
    }
}

/// Makes and returns a header object for the text of a file
pub fn get_header(content: &str, name: &str) -> Result<Header> {
    let mut mode : Option<Mode> = None;
    let mut ret = Header::empty();

    for (num, line) in content.lines().enumerate() {
        if line.is_empty() {
            continue
        }

        if match line.chars().next() {
            Some(c) => c,
            None => '?'
        } == '#' {
            // Change the mode
            if let Some(Mode::Music) = mode {
                ret.end_music = num;
            }
            mode = match &line[1..] {
                "instruments" => Some(Mode::Instruments),
                "signature" => Some(Mode::Signature),
                "music" => {
                    ret.begin_music = num + 1;
                    Some(Mode::Music)
                },
                _ => None,
            };
        }

        else {
            let mut items = line.split_whitespace();
            match mode {
                None => continue,
                Some(ref m) => match m {
                    Mode::Instruments => {
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
                    }
                    Mode::Signature => {
                        let key_name = match items.next() {
                            None => continue,
                            Some(name) => name,
                        };
                        match key_name {
                            "tempo" => ret.tempo = match items.next(){
                                Some(n) => n,
                                None => return Err(ParseError::KeyWithoutValue(
                                    name.to_string(), num).into()),
                            }.parse()?,
                            _ => return Err(ParseError::InvalidKey(
                                name.to_string(), num,  key_name.to_string())
                                .into()),
                        };
                    }
                    _ => continue
                }
            }
        }
    }
    if ret.end_music == 0 {
        ret.end_music = content.lines().count();
    }
    ret.verify(name)
}

pub fn breakdown(header: &Header, content: &str, name: &str) 
-> Result<Breakdown> {
    let mut ret = Breakdown::empty();
    let mut last_time = 0;
    for (num, line) in content.lines().enumerate().skip(header.begin_music) {
        let mut items = line.split_whitespace();
        let inst_num = match items.next() {
            Some(n) => n,
            None => continue,
        }.parse::<usize>()?;
        
        let note = match items.next() {
            Some(n) => n,
            None => return Err(ParseError::InvalidValue(name.to_string(), num)
            .into()),
        }.parse::<u32>()?;

        let time = match items.next() {
            Some(n) => n,
            None => return Err(ParseError::InvalidValue(name.to_string(), num)
            .into()),
        }.parse::<u32>()?;
        if time < last_time {
            return Err(ParseError::InvalidNoteOrder(name.to_string(), num)
            .into());
        }
        else{
            last_time = time;
        }

        let note_length = match items.next() {
            Some(n) => n,
            None => return Err(ParseError::InvalidValue(name.to_string(), num)
            .into()),
        }.parse::<u32>()?;

        let vol = match items.next() {
            Some(n) => n,
            None => return Err(ParseError::InvalidValue(name.to_string(), num)
            .into()),
        }.parse::<f32>()?;
        if vol > 1.0 {
            return Err(ParseError::InvalidValue(name.to_string(), num).into());
        }
        
        let instrument = match header.instruments.get(inst_num) {
            Some(inst) => inst,
            None => return Err(ParseError::InvalidValue(name.to_string(), num)
            .into())
        };
        ret.add_note(&instrument, note, time, note_length, vol);
    }

    Ok(ret)
}