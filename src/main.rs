use filters::naive_low_pass;
use loader::AudioMaterial;

mod filters;
mod loader;
mod utils;
mod visualise;

fn main() {
    let mut audio = AudioMaterial::new("tests/alto.wav");
    audio.draw_spectre("tests/alto.wav.png");
    naive_low_pass(&mut audio);
    audio.write("target/alto_lowpassed_naive.wav");
    audio.draw_spectre("target/alto_lowpassed_naive.wav.png");
}
