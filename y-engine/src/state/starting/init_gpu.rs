use std::sync::Arc;

use crossbeam::channel::Sender;
use wgpu::*;
use winit::window::Window;

use super::StateMsg;

/// Initialize the wgpu stuff.
pub fn init_gpu(msg_tx: Sender<StateMsg>, window: Arc<Window>) {
    let instance = Instance::new(&InstanceDescriptor {
        backends: Backends::PRIMARY,
        flags: InstanceFlags::debugging(),
        backend_options: BackendOptions::default(),
    });

    let surface = unsafe {
        instance
            .create_surface_unsafe(SurfaceTargetUnsafe::from_window(&window).unwrap())
            .unwrap()
    };

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .unwrap();

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Y-ENGINE GPU Device"),
            memory_hints: Default::default(),
        },
        None,
    ))
    .unwrap();

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);
    let size = window.inner_size();
    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::AutoVsync,
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &surface_config);

    let _ = msg_tx.send(StateMsg::InitializedGpu {
        instance,
        adapter,
        device,
        queue,
        surface,
        surface_config,
    });

    // So the state checks for messages.
    window.request_redraw();
}
