use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};

pub struct RenderCore {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
}
