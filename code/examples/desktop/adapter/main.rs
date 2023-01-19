use wgpu::{Instance, Adapter};
use winit::{
    event::{Event, WindowEvent},
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

pub async fn get_adapter(instance: &Instance) -> Option<Adapter> {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
}


pub fn handle_events(event: Event<()>, control_flow: &mut ControlFlow, window: &Window) {
    match event {
        Event::WindowEvent {
            event,
            window_id,
        } => {
            if window_id == window.id() {
                match event{
                    WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    },
                    _ => {}
                };
            }
        }
        _ => {}
    };
}

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = match create_window(&event_loop) {
        Some(window) => window, 
        None => panic!("Unable to create window")
    };

    let instance: Instance = wgpu::Instance::new(wgpu::Backends::all());
    let adapter = match get_adapter(&instance).await {
        Some(adapter) => {
            adapter
        }
        None => panic!("Unable to find a suitable WPGU adapter")
    };
    println!("Adapter loaded: {:?}", adapter);

    event_loop.run(move | event: Event<()>, _, control_flow: &mut ControlFlow | {
        handle_events(event, control_flow, &window);
    });
}

fn main() {
    env_logger::init();
    pollster::block_on(run());
}