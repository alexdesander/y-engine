use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

mod state;

#[derive(Default)]
pub struct YEngine {
    inner: Option<YEngineInner>,
}

impl ApplicationHandler for YEngine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.inner.is_none() {
            self.inner = Some(YEngineInner::new(event_loop));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        if let Some(inner) = self.inner.as_mut() {
            inner.handle_window_event(event_loop, event);
        }
    }
}

impl YEngine {
    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(self).unwrap();
    }
}

struct YEngineInner {
    state: state::State,
}

impl YEngineInner {
    fn new(event_loop: &ActiveEventLoop) -> Self {
        Self {
            state: state::State::new(event_loop),
        }
    }

    fn handle_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        self.state.handle_window_event(event_loop, event);
    }
}
