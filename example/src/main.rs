use std::sync::Arc;

use winit::{event_loop::ActiveEventLoop, window::Window};
use y_engine::{app::App, state::common::render::RenderCore, YEngine};

struct MyApp {
    _window: Arc<Window>,
    _render_core: RenderCore,
}

impl App for MyApp {
    fn window_resized(&mut self, width: u32, height: u32) {
        println!("Window resized to {}x{}", width, height);
    }

    fn window_close_requested(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
    }

    fn new(window: Arc<Window>, _render_core: RenderCore) -> Box<Self> {
        window.set_resizable(true);
        window.set_decorations(true);
        window.set_title("Y-ENGINE EXAMPLE");

        Box::new(MyApp {
            _window: window,
            _render_core,
        })
    }
}

fn main() {
    let mut yengine: YEngine<MyApp> = YEngine::default();
    yengine.run();
}
