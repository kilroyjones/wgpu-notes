mod events;
mod graphics;
mod state;


use events::handle_events;
use state::State;
use wgpu::{Instance, Adapter};
use winit::{
    event::{Event},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder}
};

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

pub async fn run() {
    let event_loop = EventLoop::new();

    let window = match create_window(&event_loop) {
        Some(window) => window, 
        None => panic!("Unable to create window")
    };
    
    #[allow(unused_variables)]
    let mut state  = State::new(&window).await;

    event_loop.run(move | event: Event<()>, _, control_flow: &mut ControlFlow | {
        handle_events(event, control_flow, &window, &mut state);
    });
}

fn main() {
    env_logger::init();
    pollster::block_on(run());
}