use std::{error, fmt, io};


pub type Result<T> = std::result::Result<T, Box<dyn error::Error + 'static>>;


#[derive(Debug)]
pub enum ParseError {
    InvalidMode(String, usize),
    NoModeDeclared(String, usize),
    InvalidSound(String, usize),
    InvalidDeclaration(String, usize),
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
            ParseError::InvalidDeclaration(name, num) =>  write!(f,
                "Instrument {} line {}: Invalid declaration.", name, num),
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