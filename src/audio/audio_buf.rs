use cpal;
use cpal::{FromSample, Sample, SizedSample};
use minimp3::{Decoder, Error, Frame};
use rubato::{InterpolationParameters, InterpolationType, Resampler, SincFixedIn, WindowFunction};
use std::io::Read;

pub struct AudioBufWriter<R: Read> {
    reader: Decoder<R>,
    channels: usize,
    resampler: Option<SincFixedIn<f32>>,
}

impl<R: Read> AudioBufWriter<R> {
    pub fn new(channels: usize, reader: Decoder<R>) -> Self {
        Self {
            reader,
            channels,
            resampler: None,
        }
    }

    pub fn fill_buf<T>(&mut self, output: &mut [T])
    where
        T: SizedSample + FromSample<u32>,
    {
        use rand::RngCore;

        for frame in output.chunks_mut(self.channels) {
            let mut rng = rand::thread_rng();
            let value = rng.next_u32().to_sample();
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }
}
