use wgpu::util::DeviceExt;
use winit::window::Window;

use super::{RenderObject, Vertex};

pub struct Renderer {
    window: Window,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub async fn new(window: Window) -> Result<Self, crate::error::Error> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }
            .or(Err(crate::error::Error::WgpuSurfaceCreateError))?;
        unsafe { instance.create_surface(&window) }
            .or(Err(crate::error::Error::WgpuSurfaceCreateError))?;

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

        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/default.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::desc(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Ok(Self {
            window,
            size,
            config,
            surface,
            device,
            queue,
            render_pipeline,
        })
    }

    pub fn render(
        &self,
        render_objects: &Vec<Box<dyn RenderObject>>,
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

        let (vertex_buffer, index_buffer, vertices_len, _indices_len) = self.get_buffers(render_objects);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

            render_pass.set_pipeline(&self.render_pipeline);

            match index_buffer {
                Some(_index_buffer) => todo!("allow usage of index buffer"),
                None => {
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.draw(0..vertices_len as u32, 0..1)
                }
            }

            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn get_buffers(
        &self,
        render_objects: &Vec<Box<dyn RenderObject>>,
    ) -> (wgpu::Buffer, Option<wgpu::Buffer>, usize, usize) {
        let mut vertex_buffer: Vec<Vertex> = vec![];
        let mut vertices_len: usize = 0;
        let index_buffer: &[u16] = &[];
        #[allow(unused_mut)]
        let mut use_index_buffer = false;

        for render_object in render_objects {
            let (vertices, indices, vertex_len, _index_len) = render_object.to_vertices();

            if let Some(_) = indices {
                todo!("Implement index buffers");
            };

            vertices_len += vertex_len;
            vertex_buffer.append(&mut (vertices.clone()));
        }

        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_buffer),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = if use_index_buffer {
                Some(self.device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(index_buffer),
                        usage: wgpu::BufferUsages::INDEX,
                    }
                ))
        } else {
            None
        };

        (vertex_buffer, index_buffer, vertices_len, 0)
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

    pub fn window(&self) -> &Window {
        &self.window
    }
}
