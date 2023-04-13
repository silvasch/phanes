use super::RenderObject;

pub struct Renderer {
    render_objects: Vec<Box<dyn RenderObject>>,
}

impl Renderer {
    pub fn new() -> RendererBuilder {
        RendererBuilder::new()
    }

    pub fn run(self) -> anyhow::Result<()> {
        todo!()
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
}
