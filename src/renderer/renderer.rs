use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use super::{RenderObject, RenderObjectsManager, Window};

pub struct Renderer {
    render_objects_manager: RenderObjectsManager,
    event_loop: EventLoop<()>,
    window: Window,
}

impl Renderer {
    pub fn new() -> RendererBuilder {
        RendererBuilder::new()
    }

    pub fn run(mut self) -> Result<(), crate::error::Error> {
        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.window().id() => match event {
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
                    WindowEvent::Resized(physical_size) => self.window.resize(*physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.window.resize(**new_inner_size)
                    }
                    _ => {}
                },
                Event::RedrawRequested(window_id) if window_id == self.window.window().id() => {
                    self.render_objects_manager.update();
                    match self
                        .window
                        .render(&self.render_objects_manager.render_objects())
                    {
                        Ok(_) => {}
                        Err(crate::error::Error::WgpuSurfaceLost) => self.window.reconfigure(),
                        Err(crate::error::Error::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    self.window.window().request_redraw();
                }
                _ => {}
            });
    }
}

pub struct RendererBuilder {
    render_objects: Vec<Box<dyn RenderObject>>,
}

impl RendererBuilder {
    pub fn new() -> Self {
        Self {
            render_objects: vec![],
        }
    }

    pub fn with_render_object(mut self, render_object: Box<dyn RenderObject>) -> Self {
        self.render_objects.push(render_object);
        return self;
    }

    pub async fn build(self) -> Result<Renderer, crate::error::Error> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .build(&event_loop)
            .or(Err(crate::error::Error::WinitOsError))?;

        Ok(Renderer {
            render_objects_manager: RenderObjectsManager::new(self.render_objects),
            event_loop,
            window: Window::new(window).await?,
        })
    }
}
