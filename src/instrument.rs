use crate::errors::{Result, ParseError};
use std::{fs, str};
use std::ops::Mul;
use std::collections::HashMap;

/// Describes how the frequency is smeared out along the CQT
#[derive(Clone)]
pub enum Smear {
    Delta(f32),
    Gaussian(f32, f32),
}

impl Mul<f32> for Smear {
    type Output = Self;

    fn mul(self, m: f32) -> Self::Output {
        match self {
            Smear::Delta(f) => Smear::Delta(f * m),
            Smear::Gaussian(f, s) => Smear::Gaussian(f * m, s * m),
        }
    }
}

/// Contains complete information about one frequency node
pub struct Sound {
    freq: Smear,
    vol: f32,
}

impl Sound {
    /// Make a new sound.
    pub fn new(freq: Smear, vol: f32) -> Sound {
        Sound {freq, vol}
    }

    /// Get the power of the sound's frequency at frequency `freq`
    pub fn get_power(&self, freq: f32) -> f32 {
        self.vol * match self.freq {
            Smear::Delta(mean) => {
                if f32::abs(freq - mean) / mean < 0.001 {
                    1.0
                }
                else {
                    0.0
                }
            },
            Smear::Gaussian(mean, sigma) => {
                f32::exp(f32::powf(mean - freq, 2.0) / (2.0 * sigma * sigma)) / 
                (sigma * 2.50662827463)
            },
        }
    }
}

pub struct Instrument {
    steady_mult: Vec<Sound>, // Ring-down time in seconds
    reverb: f32, // Ring-down time in seconds
    vol: f32, // Volume of the instrument
}

impl Instrument {
    /// Make an instrument from a file `name` with assigned volume `vol`.
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

    /// Verify the instrument after its initialization. Conumes self and returns
    /// it.
    fn verify(self, name: &str) -> Result<Instrument> {
        if self.steady_mult.len() == 0 || self.reverb == 0.0 || 
        self.vol < 0.0 || self.vol > 1.0 {
            return Err(ParseError::ModeNotHit(name.to_string()).into());
        }
        Ok(self)
    }

    /// Generate the steady-state sounds of the instrument for a given frequency
    /// `freq` and volume `vol`.
    pub fn generate_steady_state(&self, freq:f32,  vol: f32) -> Vec<Sound> {
        let mut ret = Vec::new();
        for s in &self.steady_mult {
            ret.push(Sound{freq: s.freq.clone() * freq,
                            vol: s.vol * vol * self.vol });
        }
        ret
    }
}

/// Get the contents of prefab instruments
fn get_prefab_text(name: &str) -> Option<&str> {
    match name {
        "sine" => Some(include_str!("instruments/sine.inst")),
        _ => None
    }
}

/// Mode for parsing instrument files
enum Mode {
    Steady,
    End
}

/// Make an instrument from a file
fn create_instrument(lines: &str, name: & str, vol: f32)
-> Result<Instrument> {
    let mut mode : Option<Mode> = None;
    let mut ret = Instrument { steady_mult:Vec::new(), reverb: 0.0, vol };

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
                "end" => Some(Mode::End),
                _ => return Err(
                    ParseError::InvalidMode(name.to_string(), num).into()),
            };
        }

        else {
            match mode {
                None => return Err(
                    ParseError::NoModeDeclared(name.to_string(), num).into()),
                Some(ref m) => match m {
                    Mode::Steady => {
                        let mut items = line.split_whitespace();
                        let vol = match items.next() {
                            Some(s) => s,
                            None => return Err(ParseError::InvalidSound(
                                name.to_string(), num).into()),
                        }.parse::<f32>()?;
                        let smear = match items.next() {
                            Some(s) => s,
                            None => return Err(ParseError::InvalidSound(
                                name.to_string(), num).into()),
                        };
                        let freq = match items.next() {
                            Some(s) => s,
                            None => return Err(ParseError::InvalidSound(
                                name.to_string(), num).into()),
                        }.parse::<f32>()?;
                        ret.steady_mult.push(Sound {
                            freq: match smear{
                                "Delta" => Smear::Delta(freq),
                                "Gaussian" => {
                                    let sigma = match items.next() {
                                        Some(s) => s,
                                        None => return Err(ParseError::
                                            InvalidSound(name.to_string(), num)
                                            .into()),
                                    }.parse::<f32>()?;
                                    Smear::Gaussian(freq, sigma)
                                },
                                _ => return Err(ParseError::InvalidSound(
                                    name.to_string(), num).into()),
                            },
                            vol,
                        });
                    }
                    Mode::End => {
                        let mut items = line.split_whitespace();
                        match match items.next() {
                            Some(s) => s,
                            None => continue
                        } {
                            "reverb-time" => { 
                                ret.reverb = match items.next() {
                                    Some(n) => n,
                                    None => return Err(ParseError::InvalidValue(
                                        name.to_string(), num).into())
                                }.parse()?;
                            },
                            _ => return Err(ParseError::InvalidKey(
                                name.to_string(), num).into())
                        }
                    }
                }
            }
        }
    }
    ret.verify(name)
}


/// Struct for memoizing all damp values
pub struct Damp {
    memo: HashMap<(u32, u32), f32>,
}
/// Returns the prefactor used to damp the pitch after a note is played, where
/// `t` goes from 0 (no damping) to 1 (all damping).

impl Damp {
    pub fn new() -> Damp {
        Damp {memo: HashMap::new() }
    }
    
    pub fn end_damp(&mut self, t: u32, dur: u32) -> f32 {
        match self.memo.get(&(t, dur)) {
            Some(d) => d.clone(),
            None => {
                let d = f32::max((dur - t) as f32, 0.0) / (dur as f32);
                self.memo.insert((t, dur), d);
                d.clone()
            }
        }
    }
}