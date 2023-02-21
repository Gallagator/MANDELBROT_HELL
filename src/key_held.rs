use std::collections::{HashSet, hash_set::Iter};
use winit::event::*;

pub struct KeyHeld {
    keymap: HashSet<VirtualKeyCode>,
}

impl KeyHeld {
    pub fn new() -> Self {
        Self {
            keymap: HashSet::new(),
        }
    }

    pub fn key_pressed(&mut self, code: VirtualKeyCode) {
        self.keymap.insert(code);
    }

    pub fn key_released(&mut self, code: &VirtualKeyCode) {
        self.keymap.remove(code);
    }

//     pub fn key_held(&self, code: &VirtualKeyCode) -> bool {
//         self.keymap.contains(&code)
//     }

    pub fn iter<'a>(&'a self) -> Iter<'a, VirtualKeyCode>{
        self.keymap.iter()
    }
}
