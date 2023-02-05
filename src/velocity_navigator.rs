use crate::camera::{Camera, self};
use crate::key_held::KeyHeld;
use crate::navigator::Navigator;

use std::{time::Duration, arch::x86_64::_mm_castpd_ps};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct VelocityNavigator {
    velocity: f32,
    key_held: KeyHeld,
}

impl VelocityNavigator {
    fn new(velocity: f32) -> Self {
        Self {
            velocity,
            key_held: KeyHeld::new(),
        }
    }
}

const DEFAULT_VELOCITY: f32 = 3.0;

impl Default for VelocityNavigator {
    fn default() -> Self {
        Self::new(DEFAULT_VELOCITY)
    }
}

impl Navigator for VelocityNavigator {
    fn give_input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => self.key_held.key_pressed(key.clone()),
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => self.key_held.key_released(key),
            _ => {}
        }
    }

    fn navigator_update(&mut self, delta: Duration, camera: &mut Camera, window_dims: [u32; 2]) {
        for keycode in self.key_held.iter() {
            let key_fun:fn([f32; 2], [u32; 2], _, _, _) -> ([f32; 2], f32) = match keycode {
                VirtualKeyCode::W | VirtualKeyCode::K | VirtualKeyCode::Up => {
                    |pos: [f32; 2], _, velocity, secs, scale| {([pos[0], pos[1] + velocity * secs * scale * 100.0], scale)}
                },
                VirtualKeyCode::A | VirtualKeyCode::H | VirtualKeyCode::Left => {
                    |pos: [f32; 2], _, velocity, secs, scale| {([pos[0] - velocity * secs * scale * 100.0, pos[1]], scale)}
                },
                VirtualKeyCode::S | VirtualKeyCode::J | VirtualKeyCode::Down => {
                    |pos: [f32; 2], _, velocity, secs, scale| {([pos[0], pos[1] - velocity * secs * scale * 100.0], scale)}
                },
                VirtualKeyCode::D | VirtualKeyCode::L | VirtualKeyCode::Right => {
                    |pos: [f32; 2], _, velocity, secs, scale| {([pos[0] + velocity * secs * scale * 100.0, pos[1]], scale)}
                },
                VirtualKeyCode::Equals | VirtualKeyCode::Plus => {
                    move |pos: [f32; 2], windim, velocity, secs, scale| {
                        let centre = [pos[0] + windim[0] as f32 * scale * 0.5, pos[1] - windim[1] as f32 * scale * 0.5];
                        let scale = scale * (1.0 - velocity * secs);
                        let top_left = [centre[0] - windim[0] as f32 * scale * 0.5, centre[1] + windim[1] as f32 * scale * 0.5];
                        (top_left, scale)
                    }
                },
                VirtualKeyCode::Minus => {
                    move |pos: [f32; 2], windim, velocity, secs, scale| {
                        let centre = [pos[0] + windim[0] as f32 * scale * 0.5, pos[1] - windim[1] as f32 * scale * 0.5];
                        let scale = scale * (1.0 + velocity * secs);
                        let top_left = [centre[0] - windim[0] as f32 * scale * 0.5, centre[1] + windim[1] as f32 * scale * 0.5];
                        (top_left, scale)
                    }
                },
                _ => { |pos: [f32; 2], _, _, _, scale| {(pos, scale)} },
            };
            let (pos, scale)= key_fun(camera.get_top_left(), window_dims, self.velocity, delta.as_secs_f32(), camera.get_scale());
            camera.set_scale(scale);
            camera.set_top_left(pos);
        }
    }
}
