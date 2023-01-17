use wgpu::{Instance, Adapter};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder}
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

pub fn create_window(event_loop: &EventLoop<()>) -> Option<Window> {
    match WindowBuilder::new().build(&event_loop) { 
        Ok(window) => Some(window),
        Err(_) => {
            // Log error 
            None 
        }
    } 

}

pub async fn get_adapter() -> Option<Adapter> {
    let instance: Instance = wgpu::Instance::new(wgpu::Backends::all());
    instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
}

pub fn handle_window_events(event: WindowEvent<'_>, control_flow: &mut ControlFlow) {
    match event {
        WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;
        },
        _ => {}
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() {
    let event_loop = EventLoop::new();

    let window = match create_window(&event_loop) {
        Some(window) => window, 
        None => panic!("Unable to create window")
    };
    
    let adapter = match get_adapter().await {
        Some(adapter) => {
            adapter
        }
        None => panic!("Unable to find a suitable WPGU adapter")
    };
    println!("Adapter loaded: {:?}", adapter);

    event_loop.run(move | event: Event<()>, _, control_flow: &mut ControlFlow | {
        match event {
            Event::WindowEvent {
                event,
                window_id,
            } => {
                if window_id == window.id() {
                    handle_window_events(event, control_flow);
                }
            }
            _ => {}
        };
    });
}

fn main() {
    cfg_if::cfg_if! {
         if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            pollster::block_on(run());
        } else {
            panic!("Error: This is is only set to run as wasm.");
        }
    }
}