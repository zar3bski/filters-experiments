use audio_visualizer::spectrum::plotters_png_file::spectrum_static_plotters_png_visualize;
use spectrum_analyzer::{
    samples_fft_to_spectrum,
    scaling::{divide_by_N, scale_to_zero_to_one},
    windows::{hamming_window, hann_window},
    FrequencyLimit,
};
use std::path::Path;
use wavers::{read, write, Samples, Wav};

use crate::utils::prev_power_of_two;
pub const BLOCK_SIZE: usize = 64;

pub struct AudioMaterial {
    pub path: String,
    pub blocks: Vec<[f32; BLOCK_SIZE]>,
    pub sample_rate: i32,
}

// Convenient wrapper of wavers::Wav for block processing
impl AudioMaterial {
    pub fn new(path: &str) -> Self {
        let (samples, sample_rate): (Samples<f32>, i32) = read::<f32, _>(path.to_owned()).unwrap();
        let mut blocks = vec![];
        let mut buffer: [f32; BLOCK_SIZE] = [0.0; BLOCK_SIZE];
        for n in 0..samples.len() {
            buffer[n % BLOCK_SIZE] = samples[n];
            //copy buffer to blocks
            if n % BLOCK_SIZE == BLOCK_SIZE - 1 {
                blocks.push(buffer.clone());
            }
            // fill remaining buffer with 0
            if n == samples.len() - 1 {
                for m in n % BLOCK_SIZE..BLOCK_SIZE {
                    buffer[m] = 0.0;
                }
                blocks.push(buffer.clone());
            }
        }

        Self {
            path: path.to_owned(),
            blocks: blocks,
            sample_rate: sample_rate,
        }
    }

    pub fn write(&self, path: &str) {
        let n_channels = 1; // TODO: variabilize
        write(path, &self.blocks.concat(), self.sample_rate, n_channels).unwrap();
    }

    pub fn draw_spectre(&self, dest_path: &str) {
        let filename = dest_path.split('/').into_iter().last().unwrap();
        let folder = dest_path
            .strip_suffix(filename)
            .unwrap()
            .strip_suffix('/')
            .unwrap();

        let samples = &self.blocks.concat();

        let no_window = &samples.iter().take(16384).cloned().collect::<Vec<_>>();
        let hamming_window = hamming_window(no_window);
        let spectrum_hamming_window = samples_fft_to_spectrum(
            &hamming_window,
            self.sample_rate as u32,
            FrequencyLimit::Max(10000.0),
            Some(&scale_to_zero_to_one),
        )
        .unwrap();

        spectrum_static_plotters_png_visualize(&spectrum_hamming_window.to_map(), folder, filename);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocks_in_audio_material() {
        let audio = AudioMaterial::new("tests/alto.wav");
        assert_eq!(audio.sample_rate, 44100);
        assert_eq!(*audio.blocks.last().unwrap().last().unwrap(), 0.0);
    }
}
