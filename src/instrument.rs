use crate::errors::{Result, ParseError};
use std::{fs, str};

pub struct Sound {
    freq: f32,
    amp: f32,
}

type SteadyState = Vec<Sound>;

pub struct Instrument {
    steady_mult: SteadyState,
    vol: f32
}

impl Instrument {
    pub fn new(name: &str, vol: f32) -> Result<Instrument> {
        let inst_contents : String;
        let file_text = match get_prefab_text(name) {
            Some(file_text) => file_text,
            None => {
                inst_contents = fs::read_to_string(
                    format!("instruments/{}.inst", name))?;
                &inst_contents[..]
            }
        };
        create_instrument(file_text, name, vol)
    }

    fn generate_steady_state(&self, sound: Sound) -> SteadyState {
        let mut ret = Vec::new();
        for s in &self.steady_mult {
            ret.push(Sound{freq: s.freq * sound.freq, amp: s.amp * sound.amp });
        }
        ret
    }
}

fn get_prefab_text(name: &str) -> Option<&str> {
    match name {
        "sine" => Some(include_str!("instruments/sine.inst")),
        _ => None
    }
}


enum Mode {
    Steady
}

fn create_instrument(lines: &str, name: & str, vol: f32)
-> Result<Instrument> {
    let mut mode : Option<Mode> = None;
    let mut ret = Instrument { steady_mult:Vec::new(), vol };

    for (num, line) in lines.lines().enumerate() {
        if line.is_empty() {
            continue
        }

        if match line.chars().next() {
            Some(c) => c,
            None => '?'
        } == '#' {
            // Change the mode
            mode = match &line[1..] {
                "steady" => Some(Mode::Steady),
                _ => return Err(
                    ParseError::InvalidMode(name.to_string(), num).into()),
            };
        }

        else {
            match mode {
                None => return Err(
                    ParseError::NoModeDeclared(name.to_string(), num).into()),
                Some(ref m) => match m {
                    Steady => {
                        let mut items = line.split_whitespace().map(
                            |s| s.parse::<f32>());
                        ret.steady_mult.push(
                            match (items.next(), items.next()) {
                                (Some(Ok(freq)), Some(Ok(amp))) => 
                                    Sound {freq, amp},
                                _ => return Err(ParseError::InvalidSound(
                                        name.to_string(), num).into()),
                            }
                        );
                    }
                }
            }
        }
    }
    Ok(ret)
}