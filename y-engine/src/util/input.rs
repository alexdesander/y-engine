use std::{collections::VecDeque, time::Instant};

use rustc_hash::FxHashSet;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase},
    keyboard::Key,
};

#[derive(Debug, Clone)]
pub enum InputEvent {
    MousePressed(MouseButton),
    MouseReleased(MouseButton),
    MouseWheel(MouseScrollDelta, TouchPhase),
    CursorMoved(PhysicalPosition<f64>),
    KeyPressed(Key),
    KeyReleased(Key),
    CursorLeftWindow,
    CursorEnteredWindow,
}

#[derive(Debug)]
pub struct InputManager {
    events: VecDeque<(Instant, InputEvent)>,
    max_events: usize,
    prune_cursor_moved_events: bool,

    pressed_keys: FxHashSet<Key>,
    pressed_mouse_buttons: FxHashSet<MouseButton>,
    cursor_position: PhysicalPosition<f64>,
    // This holds the position the cursor was at when the last
    // cursor_delta() call was made.
    cursor_position_for_delta: PhysicalPosition<f64>,
}

impl Default for InputManager {
    fn default() -> Self {
        Self {
            events: VecDeque::new(),
            max_events: 32,
            prune_cursor_moved_events: true,

            pressed_keys: FxHashSet::default(),
            pressed_mouse_buttons: FxHashSet::default(),
            cursor_position: PhysicalPosition::new(0.0, 0.0),
            cursor_position_for_delta: PhysicalPosition::new(0.0, 0.0),
        }
    }
}

impl InputManager {
    pub fn new(max_events: usize, prune_cursor_moved_events: bool) -> Self {
        Self {
            events: VecDeque::new(),
            max_events,
            prune_cursor_moved_events,

            pressed_keys: FxHashSet::default(),
            pressed_mouse_buttons: FxHashSet::default(),
            cursor_position: PhysicalPosition::new(0.0, 0.0),
            cursor_position_for_delta: PhysicalPosition::new(0.0, 0.0),
        }
    }

    pub fn handle_mouse_button_input(&mut self, button: MouseButton, state: ElementState) {
        let event = match state {
            ElementState::Pressed => {
                self.pressed_mouse_buttons.insert(button);
                InputEvent::MousePressed(button)
            }
            ElementState::Released => {
                self.pressed_mouse_buttons.remove(&button);
                InputEvent::MouseReleased(button)
            }
        };
        self.push_event(event);
    }

    pub fn handle_mouse_wheel_input(&mut self, delta: MouseScrollDelta, phase: TouchPhase) {
        self.push_event(InputEvent::MouseWheel(delta, phase));
    }

    pub fn handle_cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_position = position;
        if self.prune_cursor_moved_events {
            if let Some((_, InputEvent::CursorMoved(_))) = self.events.back() {
                self.events.pop_back();
            }
        }
        self.push_event(InputEvent::CursorMoved(position));
    }

    pub fn handle_keyboard_button_input(&mut self, key: Key, state: ElementState) {
        let event = match state {
            ElementState::Pressed => {
                self.pressed_keys.insert(key.clone());
                InputEvent::KeyPressed(key)
            }
            ElementState::Released => {
                self.pressed_keys.remove(&key);
                InputEvent::KeyReleased(key)
            }
        };
        self.push_event(event);
    }

    pub fn handle_cursor_left_window(&mut self) {
        self.push_event(InputEvent::CursorLeftWindow);
    }

    pub fn handle_cursor_entered_window(&mut self) {
        self.push_event(InputEvent::CursorEnteredWindow);
    }

    pub fn pop_event(&mut self) -> Option<(Instant, InputEvent)> {
        self.events.pop_front()
    }

    fn push_event(&mut self, event: InputEvent) {
        if self.events.len() >= self.max_events {
            return;
        }
        self.events.push_back((Instant::now(), event));
    }

    pub fn is_key_pressed(&self, key: &Key) -> bool {
        self.pressed_keys.contains(key)
    }

    pub fn set_key_pressed(&mut self, key: Key, pressed: bool) {
        if pressed {
            self.pressed_keys.insert(key);
        } else {
            self.pressed_keys.remove(&key);
        }
    }

    pub fn is_mouse_button_pressed(&self, button: &MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(button)
    }

    pub fn set_mouse_button_pressed(&mut self, button: MouseButton, pressed: bool) {
        if pressed {
            self.pressed_mouse_buttons.insert(button);
        } else {
            self.pressed_mouse_buttons.remove(&button);
        }
    }

    pub fn cursor_position(&self) -> PhysicalPosition<f64> {
        self.cursor_position
    }

    /// Returns the delta of the cursor position since the last call to this function.
    ///
    /// The delta is calculated as the difference between the current cursor position
    /// and the cursor position at the last call to this function.
    pub fn cursor_delta(&mut self) -> PhysicalPosition<f64> {
        let delta = PhysicalPosition::new(
            self.cursor_position.x - self.cursor_position_for_delta.x,
            self.cursor_position.y - self.cursor_position_for_delta.y,
        );
        self.cursor_position_for_delta = self.cursor_position;
        delta
    }
}
