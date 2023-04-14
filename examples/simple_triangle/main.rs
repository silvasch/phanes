use phanes::renderer::RenderObject;
use phanes::renderer::Vertex;
use phanes::Phanes;

struct Triangle {}

impl RenderObject for Triangle {
    fn to_vertices(&self) -> (Vec<Vertex>, Option<Vec<u16>>, usize, usize) {
        (
            vec![
                Vertex {
                    position: [0.0, 0.5, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [-0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [0.5, -0.5, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
            ],
            None,
            3,
            0,
        )
    }
}

fn main() {
    let app = pollster::block_on(
        Phanes::new()
            .with_render_object(Box::new(Triangle {}))
            .build(),
    ).unwrap();
    app.run().unwrap();
}
