use std::sync::Arc;

use winit::{event_loop::ActiveEventLoop, window::Window};

use crate::state::common::render::RenderCore;

pub trait App {
    fn new(window: Arc<Window>, render_core: RenderCore) -> Box<Self>
    where
        Self: Sized;

    // Window Events
    fn window_resized(&mut self, width: u32, height: u32);
    fn window_close_requested(&mut self, winit_event_loop: &ActiveEventLoop);
    fn window_redraw(&mut self);
}
