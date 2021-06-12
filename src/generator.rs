use crate::instrument::{Instrument, Sound, Damp};
use std::rc::Rc;
use std::borrow::Borrow;
use crate::output::{Output, FREQ_SIZE, SAMPLE_RATE};
use rustfft::num_complex::Complex;

const MEASURE_LENGTH: usize = 4;
const BD_SIZE: usize = 2 * MEASURE_LENGTH;
const MIN_FREQ: f32 = 10.0;
const MAX_FREQ: f32 = 10000.0;

fn frequency_to_index(freq: f32) -> usize {
    (FREQ_SIZE as f32 * (f32::ln(freq) - f32::ln(MIN_FREQ)) /
        (f32::ln(MAX_FREQ) - f32::ln(MIN_FREQ))) as usize
}
fn index_to_frequency(index: usize) -> f32 {
    MIN_FREQ * f32::powf(MAX_FREQ / MIN_FREQ, index as f32 / FREQ_SIZE as f32)
}

enum Note {
    SteadyState(Vec<Sound>),
    End(Vec<Sound>, f32),
}

/// Record frequency data on each time before pushed into a wav file
pub struct Breakdown {
    low_time: u32, // Minimum time in breakdown segment
    notes: [Vec<Rc<Note>>; BD_SIZE], // List of notes
    active: usize, // Location of minimum time in breakdown segment
    tempo: u32,
    output: Output,
    damp: Damp,
}

impl Breakdown {
    pub fn new(tempo: u32, output_file:&str) -> Breakdown {
        const INIT: Vec<Rc<Note>> = Vec::new();
        Breakdown {low_time: 0, notes: [INIT; BD_SIZE], active: 0, tempo,
            output: Output::new(output_file), damp: Damp::new() }
    }

    pub fn add_note(&mut self, inst: &Instrument, note: u32, time: u32,
        note_length: u32, vol: f32) {
        let duration = get_duration_from_length(note_length);
        let freq = 440.0 * f32::powf(2.0, (note - 69) as f32 /12.0);
        let steady_note = Rc::new(Note::SteadyState(
            inst.generate_steady_state(freq, vol)));
        let begin = time - self.low_time;
        for i in begin..(begin + duration) {
            let index = (i as usize + self.active) % BD_SIZE;
            self.notes[index].push(Rc::clone(&steady_note));
        }
        // TO DO: implement end of notes

        for _ in 0..begin {
            self.push();
        }
    }

    pub fn push_all(&mut self) {
        for _ in 0..BD_SIZE {
            self.push();
        }
    }

    fn push(&mut self) {
        for note in self.notes[self.active].iter() {
            let chunk_dur = self.tempo as f32 / 60.0 / (MEASURE_LENGTH as f32);
            let time_size = (SAMPLE_RATE as f32 * chunk_dur) as usize;
            let mut cqt = vec![[Complex{re:0.0, im:0.0}; FREQ_SIZE]; time_size];

            let frequencies: Vec<f32> = (0..FREQ_SIZE).map(
                |x| index_to_frequency(x)).collect();

            for t in 0..time_size {
                match note.borrow() {
                    Note::SteadyState(sounds) => {
                        for sound in sounds.iter(){
                            for w in 0..FREQ_SIZE {
                                cqt[t][w] += sound.get_power(
                                    frequencies[w]);
                            }
                        }
                    }
                    Note::End(sounds, dur) => {
                        for sound in sounds.iter(){
                            for w in 0..FREQ_SIZE {
                                cqt[t][w] += sound.get_power(frequencies[w]) * 
                                    self.damp.end_damp(
                                        t as u32, 
                                        (dur * SAMPLE_RATE as f32) as u32);
                            }
                        }
                    }
                }
            };

            self.output.write(&mut cqt);
        }
        self.notes[self.active].clear();
        self.active = (self.active + 1) % BD_SIZE;
        self.low_time += 1;
    }
}

fn get_duration_from_length(note_length:u32) -> u32 {
    BD_SIZE as u32 / note_length
}