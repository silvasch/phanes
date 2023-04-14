use crate::renderer::{RenderObject, Vertex};

pub struct Triangle {
    corners: (
        [f32; 3],
        [f32; 3],
        [f32; 3],
    ),
    color: [f32; 3],
}

impl Triangle {
    pub fn new(corners: ([f32; 3], [f32; 3], [f32; 3]), color: [f32; 3]) -> Self {
        Self {
            corners,
            color,
        }
    }
}

impl RenderObject for Triangle {
    fn to_vertices(&self) -> (Vec<crate::renderer::Vertex>, Option<Vec<u16>>, usize, usize) {
        (
            vec![
                Vertex {
                    position: self.corners.0,
                    color: self.color,
                },
                Vertex {
                    position: self.corners.1,
                    color: self.color,
                },
                Vertex {
                    position: self.corners.2,
                    color: self.color,
                }
            ],
            None,
            3,
            0,
        )
    }
}
