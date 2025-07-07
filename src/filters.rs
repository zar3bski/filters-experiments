use std::{collections::VecDeque, ops::Mul};

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

fn low_pass_comb_filter(audio: &mut AudioMaterial) {// TODO: implement
    //https://www.dsprelated.com/freebooks/filters/Analysis_Digital_Comb_Filter.html
    let g1: f32 = 0.5_f32.powf(3.0);
    let g2: f32 = 0.9_f32.powf(5.0);
    let mut backward_buffer: VecDeque<f32> = VecDeque::with_capacity(4);
    backward_buffer.make_contiguous();
    backward_buffer.extend([1.0, 0.0, 0.0, g1]);
    let mut forward_buffer: VecDeque<f32> = VecDeque::with_capacity(4);
    forward_buffer.make_contiguous();
    forward_buffer.extend([1.0, 0.0, 0.0, 0.0, 0.0, g2]);

    println!("backward_buffer: {:?}", backward_buffer);
    println!("forward_buffer: {:?}", forward_buffer);

    for block_nb in 0..audio.blocks.len() {
        let block = audio.blocks[block_nb];
        let mut new_block: [f32; BLOCK_SIZE] = [0.0; BLOCK_SIZE];

        for n in 0..block.len() {
            new_block[n] = block[n]; // TODO: implement

            // "move" the buffers
            backward_buffer.pop_front();
            backward_buffer.push_back(block[n]);

            forward_buffer.pop_front();
            //forward_buffer.push_back(value); // Comment on fait, à la fin du block?
        }
        audio.blocks[block_nb] = new_block
    }
}

fn MIFIR(audio: Wav<f32>) {
    //https://www.dsprelated.com/showarticle/1304.php // TODO LATER
    let fs = audio.sample_rate();
}

//
// x_* hold the previous inputs
// y_* hold the previous outputs
// b* : the zeros (feedforward coefs)
// a* : the poles (feedback coefs)
// https://en.wikipedia.org/wiki/Digital_biquad_filter#Direct_form_1
pub struct BiquadVars{
    pub x_1:f32,
    pub x_2:f32,
    pub y_1:f32,
    pub y_2:f32,
    pub a1:f32,
    pub a2:f32,
    pub b0: f32, 
    pub b1:f32,
    pub b2:f32,
}

fn biquad_filter(audio: &mut AudioMaterial, vars: &mut BiquadVars) {
    //https://www.dsprelated.com/freebooks/filters/Elementary_Filter_Sections.html
    for block_nb in 0..audio.blocks.len() {
        let block = audio.blocks[block_nb];
        let mut new_block: [f32; BLOCK_SIZE] = [0.0; BLOCK_SIZE];

        for n in 0..block.len() {
            let mut result = vars.b0 * block[n];
            result += vars.b1 * vars.x_1;
            result += vars.b2 * vars.x_2;
            result -= vars.a1 * vars.y_1;
            result -= vars.a2 * vars.y_2;
            // input sliding
            vars.x_2 = vars.x_1;
            vars.x_1 = block[n];
            // output sliding
            vars.y_2 = vars.y_1;
            vars.y_1 = result;
            new_block[n] = result
        }
        audio.blocks[block_nb] = new_block
    }

}



#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::utils::biquad_low_pass_params_generator;

    use super::*;

    #[test]
    fn test_naive_filter() {
        let mut audio = AudioMaterial::new("tests/white-noise.wav");
        naive_low_pass(&mut audio);
        audio.write("target/white-noise_lowpassed_naive.wav")
    }

    #[test]
    fn test_single_pole_low_pass_filter() {
        let mut audio = AudioMaterial::new("tests/white-noise.wav");
        single_pole_low_pass_filter(&mut audio, 50);
        audio.write("target/white-noise_single_pole_low_pass_filter.wav")
    }
    #[test]
    fn test_low_pass_comb_filter() {
        let mut audio = AudioMaterial::new("tests/white-noise.wav");
        low_pass_comb_filter(&mut audio);
        audio.write("target/white-noise_low_pass_comb_filter.wav")
    }

    #[test]
    fn test_biquad_filter() {
        let mut audio = AudioMaterial::new("tests/alto.wav");
        let mut vars = biquad_low_pass_params_generator(audio.sample_rate, 500.0);
        biquad_filter(&mut audio, &mut vars);
        audio.write("target/alto_biquad_filter.wav")
    }
}
