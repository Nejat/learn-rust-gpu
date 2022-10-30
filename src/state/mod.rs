use wgpu::{Color, Device, Queue, Surface, SurfaceConfiguration};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ModifiersState, MouseButton};

mod initialize;
mod state_impl;
mod state_static;

pub struct State {
    clear_color: Color,
    cursor_position: Option<PhysicalPosition<f64>>,
    device: Device,
    kb_state: ModifiersState,
    mouse_input: Option<MouseButton>,
    queue: Queue,
    size: PhysicalSize<u32>,
    surface: Surface,
    surface_configuration: SurfaceConfiguration,
}
