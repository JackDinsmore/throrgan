use std::fs::File;
use std::path::Path;
use rustfft::{FftPlanner, num_complex::Complex};

pub const FREQ_SIZE: usize = 1024;
pub const SAMPLE_RATE: u32 = 44_100;

pub struct Output {
    out_file: File,
    header: wav::Header
}

impl Output {
    pub fn new(output_dir: &str) -> Output {
        let header = wav::Header {
            audio_format: 1,
            channel_count: 1, // If you do 2 channels, repeat right left right
                                // left in the data.
            sampling_rate: SAMPLE_RATE,
            bytes_per_second: 88200,
            bytes_per_sample: 2,
            bits_per_sample: 16,
        };
        Output {out_file: File::create(Path::new(output_dir)).unwrap(), header }
    }

    pub fn write(&mut self, cqt: &mut Vec<[Complex<f32>; FREQ_SIZE]>) {

        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_inverse(FREQ_SIZE);

        println!("{:?}", cqt[0]);
        fft.process(&mut cqt[0]);
        println!("{:?}", cqt[0]);
        println!("");

        // TO DO: Make sure resultant fft is real, then write into data.
        // TO DO: Update docs.

        let data = wav::BitDepth::Sixteen((0..SAMPLE_RATE).map(
            |x| (25000.0 * f32::sin(x as f32 / (SAMPLE_RATE as f32) * 
                440.0 * 6.28)) as i16).collect());

        wav::write(self.header, &data, &mut self.out_file).unwrap();
    }
}