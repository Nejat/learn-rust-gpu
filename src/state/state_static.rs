use wgpu::{Backends, Instance};
use winit::window::Window;

use crate::state::initialize::{configure_surface, request_adapter, request_device};
use crate::state::State;

impl State {
    // Creating some of the wgpu types requires async code
    #[cfg_attr(target_arch = "wasm32", allow(clippy::future_not_send))] // todo: winit window is not send
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = request_adapter(&instance, &surface).await;
        let (device, queue) = request_device(&adapter).await;
        let surface_configuration = configure_surface(&adapter, &device, &surface, size);

        Self {
            surface,
            device,
            queue,
            surface_configuration,
            size,
        }
    }
}
