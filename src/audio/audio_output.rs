use super::AudioBufWriter;

use cpal;
use cpal::{
    traits::{DeviceTrait, HostTrait},
    FromSample, Sample, SizedSample,
};
use minimp3::{Decoder, Error, Frame};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct AudioOutputBuilder {
    host: cpal::Host,
    device: cpal::Device,
    config: cpal::SupportedStreamConfig,
}

pub struct AudioPlayer {
    host: cpal::Host,
    device: cpal::Device,
    config: cpal::SupportedStreamConfig,
    stream: cpal::Stream,
}

impl AudioOutputBuilder {
    pub fn new() -> Result<Self, anyhow::Error> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
        let config = device.default_output_config()?;
        Ok(Self {
            host,
            device,
            config,
        })
    }

    pub fn play<P: AsRef<Path>>(self, path: P) -> Result<AudioPlayer, anyhow::Error> {
        let f = File::open(path)?;
        let channels = self.config.channels() as usize;
        let writer = AudioBufWriter::new(channels, Decoder::new(BufReader::new(f)));

        let stream = match self.config.sample_format() {
            cpal::SampleFormat::I8 => self.build_output_stream::<i8>(writer),
            cpal::SampleFormat::I16 => self.build_output_stream::<i16>(writer),
            cpal::SampleFormat::I32 => self.build_output_stream::<i32>(writer),
            cpal::SampleFormat::I64 => self.build_output_stream::<i64>(writer),
            cpal::SampleFormat::U8 => self.build_output_stream::<u8>(writer),
            cpal::SampleFormat::U16 => self.build_output_stream::<u16>(writer),
            cpal::SampleFormat::U32 => self.build_output_stream::<u32>(writer),
            cpal::SampleFormat::U64 => self.build_output_stream::<u64>(writer),
            cpal::SampleFormat::F32 => self.build_output_stream::<f32>(writer),
            cpal::SampleFormat::F64 => self.build_output_stream::<f64>(writer),
            sample_format => Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{sample_format}'"
            ))),
        }?;
        Ok(AudioPlayer {
            host: self.host,
            device: self.device,
            config: self.config,
            stream,
        })
    }

    fn build_output_stream<T>(
        &self,
        mut writer: AudioBufWriter<BufReader<File>>,
    ) -> Result<cpal::Stream, anyhow::Error>
    where
        T: SizedSample + FromSample<u32>,
    {
        let err_fn = |err| eprintln!("Error building output sound stream: {}", err);
        Ok(self.device.build_output_stream(
            &self.config.clone().into(),
            move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
                writer.fill_buf(output);
            },
            err_fn,
            None,
        )?)
    }
}
