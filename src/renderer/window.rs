use super::RenderObject;

pub struct Window {
    window: winit::window::Window,
    surface: wgpu::Surface,
    device: wgpu::Device,
    #[allow(unused)]
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl Window {
    pub async fn new(window: winit::window::Window) -> Result<Self, crate::error::Error> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }
            .or(Err(crate::error::Error::WgpuSurfaceCreateError))?;unsafe { instance.create_surface(&window) }.or(Err(crate::error::Error::WgpuSurfaceCreateError))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(crate::error::Error::WgpuAdapterCreationFailed)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .or(Err(crate::error::Error::WgpuRequestDeviceError))?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|v| v.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Ok(Self {
            window,
            size,
            config,
            surface,
            device,
            queue,
        })
    }

    pub fn render(
        &self,
        _render_objects: &Vec<Box<dyn RenderObject>>,
    ) -> Result<(), crate::error::Error> {
        let output = match self.surface.get_current_texture() {
            Ok(output) => output,
            Err(wgpu::SurfaceError::OutOfMemory) => return Err(crate::error::Error::OutOfMemory),
            Err(wgpu::SurfaceError::Lost) => return Err(crate::error::Error::WgpuSurfaceLost),
            Err(wgpu::SurfaceError::Outdated) => {
                return Err(crate::error::Error::WgpuSurfaceOutdated)
            }
            Err(wgpu::SurfaceError::Timeout) => {
                return Err(crate::error::Error::WgpuSurfaceTimeout)
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn reconfigure(&mut self) {
        self.resize(self.size);
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }
}
