use app::App;
use state::{running, State};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

pub mod app;
pub mod state;

pub struct YEngine<T: App + 'static> {
    inner: Option<YEngineInner<T>>,
}

impl<T: App> ApplicationHandler for YEngine<T> {
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

impl<T: App + 'static> Default for YEngine<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: App + 'static> YEngine<T> {
    pub fn new() -> Self {
        Self { inner: None }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(self).unwrap();
    }
}

struct YEngineInner<T: App + 'static> {
    state: state::State,
    phantom: std::marker::PhantomData<T>,
}

impl<T: App> YEngineInner<T> {
    fn new(event_loop: &ActiveEventLoop) -> Self {
        Self {
            state: state::State::new(event_loop),
            phantom: std::marker::PhantomData,
        }
    }

    fn transition_to_running(&mut self) {
        let old_state = std::mem::replace(&mut self.state, state::State::None);
        let State::Starting(starting) = old_state else {
            panic!("Expected starting state");
        };
        self.state = State::Running(running::State::new::<T>(starting));
    }

    fn handle_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        if let State::Starting(state) = &self.state {
            if state.finished() {
                self.transition_to_running();
            }
        }

        self.state.handle_window_event(event_loop, event);
    }
}
