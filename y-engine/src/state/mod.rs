use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

pub mod common;
/// When the users app is running
pub(crate) mod running;
/// Loading state (when splash screen is shown)
mod starting;

pub enum State {
    None,
    Starting(starting::State),
    Running(running::State),
}

impl State {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        Self::Starting(starting::State::new(event_loop))
    }

    pub fn handle_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match self {
            Self::Starting(state) => state.handle_window_event(event_loop, event),
            Self::Running(state) => state.handle_window_event(event_loop, event),
            Self::None => unreachable!(),
        }
    }
}
