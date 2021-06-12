use crate::instrument::{Instrument, Sound};
use std::rc::Rc;
use std::borrow::Borrow;

const MEASURE_LENGTH: usize = 4;
const BD_SIZE: usize = 2 * MEASURE_LENGTH;

enum Note {
    SteadyState(Vec<Sound>),
}

/// Record frequency data on each time before pushed into a wav file
pub struct Breakdown {
    low_time: u32, // Minimum time in breakdown segment
    notes: [Vec<Rc<Note>>; BD_SIZE], // List of notes
    active: usize, // Location of minimum time in breakdown segment
}

impl Breakdown {
    pub fn empty() -> Breakdown {
        const INIT: Vec<Rc<Note>> = Vec::new();
        Breakdown {low_time: 0, notes: [INIT; BD_SIZE], active: 0 }
    }

    pub fn add_note(&mut self, inst: &Instrument, note: u32, time: u32,
        note_length: u32, vol: f32) {
        let duration = get_duration_from_length(note_length);
        let freq = 440.0 * f32::powf(2.0, (note - 69) as f32 /12.0);
        let note = Rc::new(Note::SteadyState(
            inst.generate_steady_state(Sound::new(freq, vol))));
        let begin = time - self.low_time;
        for i in begin..(begin + duration) {
            let index = (i as usize + self.active) % BD_SIZE;
            self.notes[index].push(Rc::clone(&note));
        }

        for _ in 0..begin {
            self.push();
        }
    }

    fn push(&mut self){
        println!("Iterating pushing with active index {}", self.active);
        for note in self.notes[self.active].iter() {
            println!("Strong note count: {}", Rc::strong_count(&note));
            match note.borrow() {
                Note::SteadyState(sounds) => {
                    // TO DO: Actually generate the waveforms
                }
            };
        }
        self.notes[self.active].clear();
        self.active = (self.active + 1) % BD_SIZE;
        self.low_time += 1;
    }
}

fn get_duration_from_length(note_length:u32) -> u32 {
    BD_SIZE as u32 / note_length
}