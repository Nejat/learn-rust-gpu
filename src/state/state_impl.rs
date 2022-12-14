use std::iter::once;

#[allow(clippy::wildcard_imports)]
use wgpu::*;
use wgpu::LoadOp::Clear;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;

use crate::state::State;

impl State {
    #[allow(clippy::unused_self)] // todo: appease clippy temporarily
    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn reconfigure_surface(&self) {
        self.surface.configure(&self.device, &self.surface_configuration);
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("render encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: Clear(Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0 }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(once(encoder.finish()));

        output.present();

        Ok(())
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.surface_configuration.width = new_size.width;
            self.surface_configuration.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_configuration);
        }
    }

    #[allow(clippy::unused_self)] // todo: appease clippy temporarily
    pub fn update(&mut self) {}
}