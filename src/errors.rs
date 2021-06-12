use std::{error, fmt};


pub type Result<T> = std::result::Result<T, Box<dyn error::Error + 'static>>;


#[derive(Debug)]
pub enum ParseError {
    InvalidMode(String, usize),
    NoModeDeclared(String, usize),
    InvalidSound(String, usize),
    ModeNotHit(String),
    KeyWithoutValue(String, usize),
    InvalidValue(String, usize),
    InvalidNoteOrder(String, usize),
    InvalidKey(String, usize),
    Unknown(String, usize),
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidMode(name, num) =>  write!(f,
                "Instrument {} line {}: Invalid mode.", name, num),
            ParseError::NoModeDeclared(name, num) =>  write!(f,
                "Instrument {} line {}: No mode declared.", name, num),
            ParseError::InvalidSound(name, num) =>  write!(f,
                "Instrument {} line {}: Invalid sound format.", name, num),
            ParseError::ModeNotHit(name) =>  write!(f,
                "File {}: Not all modes were described.", name),
            ParseError::KeyWithoutValue(name, num) =>  write!(f,
                "Instrument {} line {}: Key given without value.", name, num),
            ParseError::InvalidValue(name, num) =>  write!(f,
                "Instrument {} line {}: An invalid value was encountered.",
                name, num),
            ParseError::InvalidNoteOrder(name, num) =>  write!(f,
                "Instrument {} line {}: Invalid note order.",
                name, num),
            ParseError::InvalidKey(name, num) =>  write!(f,
                "Instrument {} line {}: Key is invalid.", name, num),
            ParseError::Unknown(name, num) =>  write!(f,
                "Instrument {} line {}: Unknown error.", name, num),
            // TO DO: Implement file and line numbers.
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            _ => None
        }
    }
}