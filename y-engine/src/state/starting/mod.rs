use std::{num::NonZeroU32, sync::Arc};

use image::RgbaImage;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Window, WindowAttributes, WindowLevel},
};

use super::common::render::RenderCore;

const SPLASH_IMAGE_RAW: &[u8] = include_bytes!("../../../assets/splashscreen.png");

mod init_gpu;

pub enum StateMsg {
    InitializedGpu {
        instance: wgpu::Instance,
        adapter: wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue,
        surface: wgpu::Surface<'static>,
        surface_config: wgpu::SurfaceConfiguration,
    },
}

pub struct State {
    pub msg_tx: crossbeam::channel::Sender<StateMsg>,
    pub msg_rx: crossbeam::channel::Receiver<StateMsg>,

    pub init_gpu_thread: Option<std::thread::JoinHandle<()>>,

    pub render_core: Option<RenderCore>,

    pub splash_image: RgbaImage,
    pub _softbuffer_context: softbuffer::Context<Arc<Window>>,
    pub softbuffer_surface: softbuffer::Surface<Arc<Window>, Arc<Window>>,
    pub window: Arc<Window>,
}

impl State {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        event_loop.set_control_flow(ControlFlow::Wait);

        let splash_image = image::load_from_memory(SPLASH_IMAGE_RAW)
            .unwrap()
            .to_rgba8();

        let monitor_size = event_loop.primary_monitor().unwrap().size();
        let (window_width, window_height) =
            (splash_image.width().max(4), splash_image.height().max(4));
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_transparent(true)
                    .with_decorations(false)
                    .with_inner_size(PhysicalSize::new(window_width, window_height))
                    .with_position(PhysicalPosition::new(
                        (monitor_size.width - window_width) / 2,
                        (monitor_size.height - window_height) / 2,
                    ))
                    .with_window_level(WindowLevel::AlwaysOnTop)
                    .with_title("Y-ENGINE"),
            )
            .unwrap();
        let window = Arc::new(window);

        let softbuffer_context = softbuffer::Context::new(window.clone()).unwrap();
        let softbuffer_surface =
            softbuffer::Surface::new(&softbuffer_context, window.clone()).unwrap();

        let (msg_tx, msg_rx) = crossbeam::channel::unbounded();

        let mut s = Self {
            msg_tx,
            msg_rx,
            init_gpu_thread: None,
            render_core: None,
            splash_image,
            _softbuffer_context: softbuffer_context,
            softbuffer_surface,
            window,
        };

        s.spawn_init_threads();

        s
    }

    pub fn handle_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        while let Ok(msg) = self.msg_rx.try_recv() {
            match msg {
                StateMsg::InitializedGpu {
                    instance,
                    adapter,
                    device,
                    queue,
                    surface,
                    surface_config,
                } => {
                    assert!(self.render_core.is_none());
                    self.render_core = Some(RenderCore {
                        instance,
                        adapter,
                        device,
                        queue,
                        surface,
                        surface_config,
                    });
                }
            }
            // So the engine checks if the transition from starting to running should happen
            self.window.request_redraw();
        }
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                let (width, height) = (size.width, size.height);
                if width == 0 || height == 0 {
                    return;
                }

                self.softbuffer_surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();
                self.draw_y_engine_splash();
            }
            _ => {}
        }
    }

    pub fn finished(&self) -> bool {
        self.render_core.is_some()
    }

    fn draw_y_engine_splash(&mut self) {
        let size = self.window.inner_size();
        let mut buffer = self.softbuffer_surface.buffer_mut().unwrap();
        for index in 0..(size.width * size.height) {
            let color = self
                .splash_image
                .get_pixel(index % size.width, index / size.width);
            buffer[index as usize] = u32::from_ne_bytes(color.0);
        }
        buffer.present().unwrap();
    }

    fn spawn_init_threads(&mut self) {
        assert!(self.init_gpu_thread.is_none());
        let _window = self.window.clone();
        let _msg_tx = self.msg_tx.clone();
        self.init_gpu_thread = Some(std::thread::spawn(|| init_gpu::init_gpu(_msg_tx, _window)));
    }
}
