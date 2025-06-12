use std::ops::Mul;

use wavers::{Samples, Wav};

use crate::loader::{AudioMaterial, BLOCK_SIZE};

pub fn naive_low_pass(audio: &mut AudioMaterial) {
    // https://www.dsprelated.com/freebooks/filters/Simplest_Lowpass_Filter_I.html

    let mut last_sample: f32 = 0.0;

    for block_nb in 0..audio.blocks.len() {
        let block = audio.blocks[block_nb];
        let mut new_block: [f32; BLOCK_SIZE] = [0.0; BLOCK_SIZE];

        new_block[0] = block.first().unwrap() + last_sample;

        for n in 1..block.len() {
            new_block[n] = block[n] + block[n - 1];
        }
        last_sample = block.last().unwrap().clone();

        audio.blocks[block_nb] = new_block
    }
}

fn single_pole_low_pass_filter(audio: &mut AudioMaterial, cutoff_frequency: u16) {
    // https://www.embeddedrelated.com/showarticle/779.php
    // https://dsp.stackexchange.com/a/54088
    let y = 1 as f32 - f32::cos(cutoff_frequency as f32);

    let alpha = -y + f32::sqrt(y.powf(2.0) + y.mul(2.0));
    let mut last_sample: f32 = *audio.blocks.first().unwrap().first().unwrap();
    for block_nb in 0..audio.blocks.len() {
        let block = audio.blocks[block_nb];
        let mut new_block: [f32; BLOCK_SIZE] = [0.0; BLOCK_SIZE];

        for n in 0..block.len() {
            last_sample += alpha * (block[n] - last_sample);
            new_block[n] = last_sample
        }
        audio.blocks[block_nb] = new_block
    }
}

fn MIFIR(audio: Wav<f32>) {
    //https://www.dsprelated.com/showarticle/1304.php // TODO LATER
    let fs = audio.sample_rate();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive_filter() {
        let mut audio = AudioMaterial::new("tests/alto.wav");
        naive_low_pass(&mut audio);
        audio.write("target/alto_lowpassed_naive.wav")
    }

    #[test]
    fn test_single_pole_low_pass_filter() {
        let mut audio = AudioMaterial::new("tests/alto.wav");
        single_pole_low_pass_filter(&mut audio, 50);
        audio.write("target/alto_single_pole_low_pass_filter.wav")
    }
}
