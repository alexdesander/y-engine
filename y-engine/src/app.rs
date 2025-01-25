use std::sync::Arc;

use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::Key,
    window::Window,
};

use crate::state::common::render::RenderCore;

#[allow(unused_variables)]
pub trait App {
    fn new(window: Arc<Window>, render_core: RenderCore) -> Box<Self>
    where
        Self: Sized;

    // Window events
    /// Return true if the event was consumed and should not be passed to the next handler.
    fn window_raw(&mut self, event: &WindowEvent, winit_event_loop: &ActiveEventLoop) -> bool {
        false
    }
    fn window_resized(&mut self, width: u32, height: u32) {}
    fn window_close_requested(&mut self, winit_event_loop: &ActiveEventLoop) {}
    fn window_redraw(&mut self) {}

    // User input events
    fn mouse_button_input(&mut self, button: MouseButton, state: ElementState) {}
    fn mouse_wheel_input(&mut self, delta: MouseScrollDelta, phase: TouchPhase) {}
    fn cursor_moved(&mut self, position: PhysicalPosition<f64>) {}
    fn cursor_entered_window(&mut self) {}
    fn cursor_left_window(&mut self) {}
    fn keyboard_button_input(&mut self, key: Key, state: ElementState) {}
}
