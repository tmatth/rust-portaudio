//! Play a sine wave for several seconds.
//!
//! A rusty adaptation of the official PortAudio C "paex_sine.c" example by Phil Burk and Ross
//! Bencina.

extern crate portaudio;
extern crate rand;


use rand::distributions::{IndependentSample, Range};
use portaudio as pa;


const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 200;


fn main() {
    match run() {
        Ok(_) => {},
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}


fn run() -> Result<(), pa::Error> {

    println!("PortAudio Test: output sine wave. SR = {}, BufSize = {}", SAMPLE_RATE, FRAMES_PER_BUFFER);

    // Initialise sinusoidal wavetable.
    let mut left_phase = 0;
    let mut right_phase = 0;

    let pa = try!(pa::PortAudio::new());

    let mut settings = try!(pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));
    // we won't output out of range samples so don't bother clipping them.
    settings.flags = pa::stream_flags::CLIP_OFF;

    let between = Range::new(-1f64, 1.);
    let mut rng = rand::thread_rng();
    // This routine will be called by the PortAudio engine when audio is needed. It may called at
    // interrupt level on some machines so don't do anything that could mess up the system like
    // dynamic resource allocation or IO.
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut idx = 0;
        for _ in 0..frames {
            let num: f32 = between.ind_sample(&mut rng) as f32;
            buffer[idx]   = num;
            buffer[idx+1] = num;
            left_phase += 1;
            if left_phase >= TABLE_SIZE { left_phase -= TABLE_SIZE; }
            right_phase += 3;
            if right_phase >= TABLE_SIZE { right_phase -= TABLE_SIZE; }
            idx += 2;
        }
        pa::Continue
    };

    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

    try!(stream.start());

    println!("Play for {} seconds. {}", NUM_SECONDS, left_phase);
    pa.sleep(NUM_SECONDS * 1_000);

    try!(stream.stop());
    try!(stream.close());

    println!("Test finished.");

    Ok(())
}
