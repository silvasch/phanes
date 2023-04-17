use phanes::{builtin::render_objects::Triangle, Phanes};

fn main() {
    let app = pollster::block_on(
        Phanes::new()
            .with_render_object(Box::new(Triangle::new(
                // middle
                ([0.1, 0.0, 0.0], [0.0, 0.2, 0.0], [-0.1, 0.0, 0.0]),
                [1.0, 0.0, 0.0],
            )))
            .with_render_object(Box::new(Triangle::new(
                // bottom left
                ([-1.0, -1.0, 0.0], [-0.8, -1.0, 0.0], [-1.0, -0.8, 0.0]),
                [0.0, 1.0, 0.0],
            )))
            .with_render_object(Box::new(Triangle::new(
                // bottom right
                ([1.0, -1.0, 0.0], [1.0, -0.8, 0.0], [0.8, -1.0, 0.0]),
                [0.0, 1.0, 0.0],
            )))
            .with_render_object(Box::new(Triangle::new(
                // top right
                ([1.0, 1.0, 0.0], [0.8, 1.0, 0.0], [1.0, 0.8, 0.0]),
                [0.0, 1.0, 0.0],
            )))
            .with_render_object(Box::new(Triangle::new(
                // top left
                ([-1.0, 1.0, 0.0], [-1.0, 0.8, 0.0], [-0.8, 1.0, 0.0]),
                [0.0, 1.0, 0.0],
            )))
            .build(),
    )
    .unwrap();
    app.run().unwrap();
}
