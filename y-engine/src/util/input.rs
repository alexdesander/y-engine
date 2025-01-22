use std::{collections::VecDeque, time::Instant};

use winit::{event::{ElementState, MouseButton}, keyboard::Key};


#[derive(Debug, Clone)]
pub enum InputEvent {
    MousePressed(MouseButton),
    MouseReleased(MouseButton),
    KeyPressed(Key),
    KeyReleased(Key),
}

#[derive(Debug)]
pub struct InputManager {
    events: VecDeque<(Instant, InputEvent)>,
    max_events: usize,
}

impl Default for InputManager {
    fn default() -> Self {
        Self {
            events: VecDeque::new(),
            max_events: 32,
        }
    }
}

impl InputManager {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: VecDeque::new(),
            max_events,
        }
    }

    pub fn handle_mouse_button_input(&mut self, button: MouseButton, state: ElementState) {
        let event = match state {
            ElementState::Pressed => InputEvent::MousePressed(button),
            ElementState::Released => InputEvent::MouseReleased(button),
        };
        if self.events.len() >= self.max_events {
            return;
        }
        self.events.push_back((Instant::now(), event));
    }

    pub fn handle_keyboard_button_input(&mut self, key: Key, state: ElementState) {
        let event = match state {
            ElementState::Pressed => InputEvent::KeyPressed(key),
            ElementState::Released => InputEvent::KeyReleased(key),
        };
        if self.events.len() >= self.max_events {
            return;
        }
        self.events.push_back((Instant::now(), event));
    }
}