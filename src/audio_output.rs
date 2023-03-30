use cpal;
use cpal::{
    traits::{DeviceTrait, HostTrait},
    DevicesError, FromSample, Sample, SizedSample,
};

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

    pub fn play(self) -> Result<AudioPlayer, anyhow::Error> {
        let stream = match self.config.sample_format() {
            cpal::SampleFormat::I8 => self.build_output_stream::<i8>(),
            cpal::SampleFormat::I16 => self.build_output_stream::<i16>(),
            cpal::SampleFormat::I32 => self.build_output_stream::<i32>(),
            cpal::SampleFormat::I64 => self.build_output_stream::<i64>(),
            cpal::SampleFormat::U8 => self.build_output_stream::<u8>(),
            cpal::SampleFormat::U16 => self.build_output_stream::<u16>(),
            cpal::SampleFormat::U32 => self.build_output_stream::<u32>(),
            cpal::SampleFormat::U64 => self.build_output_stream::<u64>(),
            cpal::SampleFormat::F32 => self.build_output_stream::<f32>(),
            cpal::SampleFormat::F64 => self.build_output_stream::<f64>(),
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

    fn build_output_stream<T>(&self) -> Result<cpal::Stream, anyhow::Error>
    where
        T: SizedSample + FromSample<u32>,
    {
        use rand::RngCore;
        let err_fn = |err| eprintln!("Error building output sound stream: {}", err);
        let channels = self.config.channels() as usize;
        Ok(self.device.build_output_stream(
            &self.config.clone().into(),
            move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in output.chunks_mut(channels) {
                    let mut rng = rand::thread_rng();
                    let value = rng.next_u32().to_sample();
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            err_fn,
            None,
        )?)
    }
}
