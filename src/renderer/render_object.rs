use super::Vertex;

pub trait RenderObject {
    fn update(&mut self) {}
    fn to_vertices(&self) -> Vec<Vertex>;
}
