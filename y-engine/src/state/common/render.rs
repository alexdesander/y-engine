use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};

/// Holds the core render objects like the device, queue, and surface.
pub struct RenderCore {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
}
