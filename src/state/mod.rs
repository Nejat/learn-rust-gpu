use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

mod initialize;
mod state_impl;
mod state_static;

pub struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    surface_configuration: SurfaceConfiguration,
    size: PhysicalSize<u32>,
}
