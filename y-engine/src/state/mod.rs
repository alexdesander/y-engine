use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

mod common;
mod starting;

pub enum State {
    Starting(starting::State),
}

impl State {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        Self::Starting(starting::State::new(event_loop))
    }

    pub fn handle_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match self {
            Self::Starting(state) => state.handle_window_event(event_loop, event),
        }
    }
}
