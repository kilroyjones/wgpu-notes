use wgpu::{Adapter, Device, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

pub async fn get_device_and_queue(adapter: &Adapter) -> (Device, Queue) {
   match adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        },
        None,
    ).await {
        Ok((device, queue)) => (device, queue),
        Err(e) => panic!("Unable to get device and queue {:?}", e) 
    }
}

pub fn get_surface_config(adapter: &Adapter, size: PhysicalSize<u32>, surface: &Surface) -> SurfaceConfiguration {
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_supported_formats(&adapter)[0],
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
    };
    config
}