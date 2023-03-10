use winit::window::Window;
use wgpu::{Adapter, Device, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;
pub struct State {
    
    _config: wgpu::SurfaceConfiguration,
    _device: wgpu::Device,
    _queue: wgpu::Queue,
    _size: winit::dpi::PhysicalSize<u32>,
    #[allow(dead_code)]
    surface: wgpu::Surface,
}

impl State {
    pub async fn new(window: &Window, adapter: &Adapter) -> Self {
        let _size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let (_device, _queue) = get_device_and_queue(adapter).await;
        let _config = get_surface_config(&adapter, _size, &surface);

        Self {
            _config,
            _device,
            _queue,
            _size,
            surface,
        }
    }
}

async fn get_device_and_queue(adapter: &Adapter) -> (Device, Queue) {
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
