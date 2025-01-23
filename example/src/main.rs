use std::sync::Arc;

use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase},
    event_loop::ActiveEventLoop,
    keyboard::Key,
    window::Window,
};
use y_engine::{app::App, state::common::render::RenderCore, util::input::InputManager, YEngine};

struct MyApp {
    _window: Arc<Window>,
    render_core: RenderCore,
    input_manager: InputManager,
}

impl App for MyApp {
    fn new(window: Arc<Window>, render_core: RenderCore) -> Box<Self> {
        window.set_resizable(true);
        window.set_decorations(true);
        window.set_title("Y-ENGINE EXAMPLE");

        Box::new(MyApp {
            _window: window,
            render_core,
            input_manager: InputManager::default(),
        })
    }

    fn window_resized(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.render_core.surface_config.width = width;
            self.render_core.surface_config.height = height;
            self.render_core
                .surface
                .configure(&self.render_core.device, &self.render_core.surface_config);
        }
    }

    fn window_close_requested(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
    }

    fn window_redraw(&mut self) {
        match self.render() {
            Ok(_) => {}
            Err(wgpu::SurfaceError::OutOfMemory) => panic!("Out of memory"),
            _ => {}
        }
    }

    fn mouse_button_input(&mut self, button: MouseButton, state: ElementState) {
        self.input_manager.handle_mouse_button_input(button, state);
    }

    fn keyboard_button_input(&mut self, key: Key, state: ElementState) {
        self.input_manager.handle_keyboard_button_input(key, state);
    }

    fn mouse_wheel_input(&mut self, delta: MouseScrollDelta, phase: TouchPhase) {
        self.input_manager.handle_mouse_wheel_input(delta, phase);
    }

    fn cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        self.input_manager.handle_cursor_moved(position);
    }

    fn cursor_entered_window(&mut self) {
        self.input_manager.handle_cursor_entered_window();
    }

    fn cursor_left_window(&mut self) {
        self.input_manager.handle_cursor_left_window();
    }
}

impl MyApp {
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let rc = &self.render_core;
        let output = rc.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = rc
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }
        rc.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn main() {
    let mut yengine: YEngine<MyApp> = YEngine::default();
    yengine.run();
}
