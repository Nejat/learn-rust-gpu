#[allow(clippy::wildcard_imports)]
use wgpu::*;
use winit::dpi::PhysicalSize;

pub fn configure_surface(
    adapter: &Adapter,
    device: &Device,
    surface: &Surface,
    size: PhysicalSize<u32>,
) -> SurfaceConfiguration {
    let configuration = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_supported_formats(adapter)[0],
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo,
        alpha_mode: CompositeAlphaMode::Auto,
    };

    surface.configure(device, &configuration);

    configuration
}

pub async fn request_adapter(instance: &Instance, surface: &Surface) -> Adapter {
    instance.request_adapter(
        &RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        },
    ).await.unwrap()

    // let adapter = instance
    //     .enumerate_adapters(Backends::all())
    //     .filter(|adapter| {
    //         // Check if this adapter supports our surface
    //         surface.get_preferred_format(&adapter).is_some()
    //     })
    //     .next()
    //     .unwrap();
}

pub async fn request_device(adapter: &Adapter) -> (Device, Queue) {
    adapter.request_device(
        &DeviceDescriptor {
            features: Features::empty(),
            // WebGL doesn't support all of wgpu's features, so if
            // we're building for the web we'll have to disable some.
            limits: if cfg!(target_arch = "wasm32") {
                Limits::downlevel_webgl2_defaults()
            } else {
                Limits::default()
            },
            label: None,
        },
        None, // Trace path
    ).await.unwrap()
}
