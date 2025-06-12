use audio_visualizer::{
    waveform::png_file::waveform_static_png_visualize, ChannelInterleavement, Channels,
};
use wavers::{Samples, Wav};

// simple dump
fn load() {
    let mut audio: Wav<i16> = Wav::from_path("./tests/alto.wav").unwrap();
    let samples: Samples<i16> = audio.read().unwrap();
    waveform_static_png_visualize(&samples, Channels::Mono, "target/out", "alto.png");
}

#[test]
fn test_load() {
    load();
}
