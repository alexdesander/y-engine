use std::sync::Arc;

use winit::{event::{ElementState, MouseButton}, event_loop::ActiveEventLoop, keyboard::Key, window::Window};

use crate::state::common::render::RenderCore;

pub trait App {
    fn new(window: Arc<Window>, render_core: RenderCore) -> Box<Self>
    where
        Self: Sized;

    // Window events
    fn window_resized(&mut self, width: u32, height: u32);
    fn window_close_requested(&mut self, winit_event_loop: &ActiveEventLoop);
    fn window_redraw(&mut self);
    
    // User input events
    fn mouse_button_input(&mut self, button: MouseButton, state: ElementState);
    fn keyboard_button_input(&mut self, key: Key, state: ElementState);
}
