#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]

#![allow(clippy::module_name_repetitions)]

#[macro_use]
extern crate cfg_if;

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::ControlFlow;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::init::{initialize_environment, initialize_logging};
#[cfg(target_arch = "wasm32")]
use crate::init::initialize_canvas;

mod init;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    initialize_logging();

    let (event_loop, window) = initialize_environment();

    #[cfg(target_arch = "wasm32")]
    initialize_canvas(&window);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}

