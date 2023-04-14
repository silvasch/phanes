use phanes::{builtin::render_objects::Triangle, Phanes};

fn main() {
    let app = pollster::block_on(
        Phanes::new()
            .with_render_object(Box::new(Triangle::new(
                ([0.0, 0.5, 0.0], [-0.5, -0.5, 0.0], [0.5, -0.5, 0.0]),
                [1.0, 0.0, 0.0],
            )))
            .build(),
    )
    .unwrap();
    app.run().unwrap();
}
