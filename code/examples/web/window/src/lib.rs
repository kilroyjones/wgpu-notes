use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::web::WindowExtWebSys,
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

pub fn create_canvas(window: &Window) {
    window.set_inner_size(PhysicalSize::new(450, 400));
    
    match web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let dst = doc.get_element_by_id("wasm-example")?;
            let canvas = web_sys::Element::from(window.canvas());
            dst.append_child(&canvas).ok()?;
            Some(())
        }) {
            Some(_) => return,
            None => panic!("Error: Unable to create canvas")
        }
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
pub fn run() {
    let event_loop = EventLoop::new();

    let window = match create_window(&event_loop) {
        Some(window) => window, 
        None => panic!("Unable to create window")
    };

    create_canvas(&window);
    
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
