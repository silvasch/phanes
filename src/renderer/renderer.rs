use winit::{event_loop::{EventLoop, ControlFlow}, window::{Window, WindowBuilder}, event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};

use super::RenderObject;

pub struct Renderer {
    render_objects: Vec<Box<dyn RenderObject>>,

    event_loop: EventLoop<()>,
    window: Window,
}

impl Renderer {
    pub fn new() -> RendererBuilder {
        RendererBuilder::new()
    }

    pub fn run(self) -> anyhow::Result<()> {
        self.event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => match event {
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
            }
            _ => {},
        });
    }
    
    #[allow(unused)]
    fn update(&mut self) {
        for render_object in &mut self.render_objects {
            render_object.update();
        }
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
        return self
    }

    pub fn build(self) -> anyhow::Result<Renderer> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop)?;

        Ok(Renderer {
            render_objects: self.render_objects,

            event_loop,
            window,
        })
    }
}
