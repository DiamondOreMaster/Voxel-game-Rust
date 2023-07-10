use std::{collections::HashMap, sync::mpsc::Receiver};

use glfw::WindowEvent;

extern crate glfw;

pub struct Input {
    
   pub key_state: HashMap<glfw::Key, bool>,
}
 
impl Input {
    pub fn new() -> Input {
        Input {
            key_state: HashMap::new(),
        }
    }

    pub fn handle_window_events(&mut self, events: &Receiver<(f64, WindowEvent)>) {
        for (_, event) in glfw::flush_messages(events) {
            match event {
                glfw::WindowEvent::Key(key, _, action, _) => {
                    match action {
                        glfw::Action::Press => {
                            // Set the key state to true when the key is pressed
                            self.key_state.insert(key, true);
                        },
                        glfw::Action::Release => {
                            // Set the key state to false when the key is released
                            self.key_state.insert(key, false);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    pub fn is_key_down(&self, key: glfw::Key) -> bool {
        return *self.key_state.get(&key).unwrap_or(&false)
    }
}