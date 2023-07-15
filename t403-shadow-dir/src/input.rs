use std::collections::HashSet;

use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

pub struct Input {
    press_map: HashSet<VirtualKeyCode>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            press_map: HashSet::new(),
        }
    }

    pub fn is_pressed(&self, key: VirtualKeyCode) -> bool {
        self.press_map.contains(&key)
    }

    pub fn on_input(&mut self, input: KeyboardInput) {
        if let Some(key) = input.virtual_keycode {
            if input.state == ElementState::Pressed {
                if !self.press_map.contains(&key) {
                    self.press_map.insert(key);
                }
            } else if input.state == ElementState::Released {
                if self.press_map.contains(&key) {
                    self.press_map.remove(&key);
                }
            }
        }
    }
}
