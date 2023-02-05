use crate::camera::Camera;
use crate::key_held::KeyHeld;
use crate::navigator::Navigator;

use std::time::Duration;
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

impl Default for VelocityNavigator {
    fn default() -> Self {
        Self::new(1.0)
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

    fn navigator_update(&mut self, delta: Duration, camera: &mut Camera) {
        for keycode in self.key_held.iter() {
            let key_fun:fn([f32; 2], _, _) -> [f32; 2] = match keycode {
                VirtualKeyCode::W | VirtualKeyCode::K | VirtualKeyCode::Up => {
                    |pos: [f32; 2], velocity, secs| {[pos[0], pos[1] + velocity * secs]}
                },
                VirtualKeyCode::A | VirtualKeyCode::H | VirtualKeyCode::Left => {
                    |pos: [f32; 2], velocity, secs| {[pos[0] - velocity * secs, pos[1]]}
                } 
                VirtualKeyCode::S | VirtualKeyCode::J | VirtualKeyCode::Down => {
                    |pos: [f32; 2], velocity, secs| {[pos[0], pos[1] - velocity * secs]}
                }
                VirtualKeyCode::D | VirtualKeyCode::L | VirtualKeyCode::Right => {
                    |pos: [f32; 2], velocity, secs| {[pos[0] + velocity * secs, pos[1]]}
                }
                _ => { |pos: [f32; 2], _, _| {pos} }
            };
            camera.set_top_left(key_fun(camera.get_top_left(), self.velocity, delta.as_secs_f32()));
        }
    }
}
