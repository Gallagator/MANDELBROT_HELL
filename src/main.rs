use anyhow;

mod audio;
mod camera;
mod key_held;
mod navigator;
mod state;
mod velocity_navigator;

// TODO use rust argparser
fn main() -> Result<(), anyhow::Error> {
    let args: Vec<_> = std::env::args().collect();
    let ao = audio::AudioOutputBuilder::new()?;
    let player = ao.play(&args[1])?;
    pollster::block_on(state::run());
    Ok(())
}
