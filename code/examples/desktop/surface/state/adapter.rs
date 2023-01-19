use wgpu::{Instance, Adapter, Surface};

pub async fn get_adapter(instance: &Instance, surface: &Surface) -> Result<Adapter, ()>  {
    match instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        },
    ).await { 
        Some(adapter) => adapter,
        None => return Err(())
    };      

    match is_adapter_supported(&instance, &surface).await {
        Ok(adapter) => Ok(adapter),
        Err(e) => return Err(e)
    }
}


async fn is_adapter_supported(
    instance: &Instance,
    surface: &Surface) -> Result<Adapter, ()> {
    match instance
        .enumerate_adapters(wgpu::Backends::all())
        .filter(|adapter| {
            !surface.get_supported_formats(&adapter).is_empty()
        }).next() {
        Some(adapter) => Ok(adapter),
        None => return Err(())
    }
}