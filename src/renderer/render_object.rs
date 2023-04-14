use super::Vertex;

pub trait RenderObject {
    fn update(&mut self) {}
    /// Convert the Renderobject to vertices
    /// Used by phanes to draw the object
    /// Returns vertex buffer, optional index buffer, length of the vertex buffer, length of the
    /// index buffer
    fn to_vertices(&self) -> (Vec<Vertex>, Option<Vec<u16>>, usize, usize);
}
