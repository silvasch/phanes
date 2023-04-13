use super::RenderObject;

pub struct RenderObjectsManager {
    render_objects: Vec<Box<dyn RenderObject>>,
}

impl RenderObjectsManager {
    pub fn new(render_objects: Vec<Box<dyn RenderObject>>) -> Self {
        Self { render_objects }
    }

    pub fn update(&mut self) {
        for render_object in &mut self.render_objects {
            render_object.update();
        }
    }

    pub fn render_objects(&self) -> &Vec<Box<dyn RenderObject>> {
        &self.render_objects
    }
}
