use std::sync::Arc;

use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::Window};

use crate::app::App;

use super::starting;

pub struct State {
    app: Box<dyn App>,
    _window: Arc<Window>,
}

impl State {
    pub fn new<T: App + 'static>(old_state: starting::State) -> Self {
        Self {
            app: T::new(old_state.window.clone(), old_state.render_core.unwrap()),
            _window: old_state.window,
        }
    }

    pub fn handle_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        if self.app.window_raw(&event) {
            return;
        }
        match event {
            WindowEvent::CloseRequested => {
                self.app.window_close_requested(event_loop);
            }
            WindowEvent::Resized(size) => {
                self.app.window_resized(size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                self.app.window_redraw();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.app.mouse_button_input(button, state);
            }
            WindowEvent::MouseWheel { delta, phase, .. } => {
                self.app.mouse_wheel_input(delta, phase);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.app
                    .keyboard_button_input(event.logical_key, event.state);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.app.cursor_moved(position);
            }
            WindowEvent::CursorLeft { .. } => {
                self.app.cursor_left_window();
            }
            WindowEvent::CursorEntered { .. } => {
                self.app.cursor_entered_window();
            }
            _ => {}
        }
    }
}
