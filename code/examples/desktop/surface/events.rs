use crate::State;

use winit::{
    event::*,
    event_loop::{ControlFlow},
    window::Window
};

pub fn handle_events(
    event: Event<()>, 
    control_flow: &mut ControlFlow, 
    window: &Window, 
    state: &mut State) {

    match event {
        Event::WindowEvent {
            event,
            window_id,
        } => {
            if window_id == window.id() { 
                if !state.input(&event) {
                    window_events(event, control_flow, state);
                }
            }
        }
        _ => other_events(event, control_flow, window, state)
    };
}

fn window_events(
    event: WindowEvent<'_>,
    control_flow: &mut ControlFlow, 
    state: &mut State) {
        
    match event {
        WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;
        },
        WindowEvent::Resized(physical_size) => {
            state.set_size(physical_size);
        }
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            state.set_size(*new_inner_size);
        }
        _ => {}
    }
}

fn other_events(
    event: Event<()>, 
    control_flow: &mut ControlFlow, 
    window: &Window, 
    state: &mut State) {

    match event {
        Event::RedrawRequested(window_id) => {
            if window_id == window.id() {
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit, 
                    Err(_) => { 
                        // LOG ERROR
                    },
                }
            }
        } 
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    }
}

