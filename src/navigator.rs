use crate::camera::{Camera, self};
use std::time::Duration;
use winit::event::WindowEvent;

pub trait Navigator {
    fn give_input(&mut self, event: &WindowEvent);
    fn navigator_update(&mut self, delta: Duration, camera: &mut Camera, window_dims: [u32; 2]);
}
