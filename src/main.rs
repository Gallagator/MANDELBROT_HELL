use anyhow;

mod audio_output;
mod camera;
mod key_held;
mod navigator;
mod state;
mod velocity_navigator;

fn main() -> Result<(), anyhow::Error> {
    let ao = audio_output::AudioOutputBuilder::new()?;
    let player = ao.play()?;
    pollster::block_on(state::run());
    Ok(())
}
