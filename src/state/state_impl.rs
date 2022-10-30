use std::iter::once;

#[allow(clippy::wildcard_imports)]
use wgpu::*;
use wgpu::LoadOp::Clear;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, WindowEvent};

use crate::state::State;

impl State {
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } =>
                self.cursor_position = Some(*position),
            WindowEvent::ModifiersChanged(modified_state) =>
                self.kb_state = *modified_state,
            WindowEvent::MouseInput { state: mouse_state, button, .. } =>
                if *mouse_state == ElementState::Pressed {
                    self.mouse_input = Some(*button);
                } else {
                    self.mouse_input = None;
                },
            // WindowEvent::MouseWheel { delta, phase, .. } => {}
            _ => return false
        }

        true
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
                        load: Clear(self.clear_color),
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

    pub fn update(&mut self) {
        #[allow(clippy::cast_lossless)]
        if let Some(PhysicalPosition { x, y }) = self.cursor_position {
            self.clear_color = Color {
                r: x / self.size.width as f64,
                g: (x + y) / (self.size.width + self.size.height) as f64,
                b: y / self.size.height as f64,
                a: 1.0,
            }
        }

        self.cursor_position = None;
    }
}