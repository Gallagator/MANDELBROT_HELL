mod camera;
mod key_held;
mod navigator;
mod state;
mod velocity_navigator;

fn main() {
    pollster::block_on(state::run())
}
