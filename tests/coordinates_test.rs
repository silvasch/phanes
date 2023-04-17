use phanes::{builtin::render_objects::Triangle, Phanes};

#[test]
fn coordinates_test() {
    let app = pollster::block_on(
        Phanes::new()
            .with_render_object(Box::new(Triangle::new(
                ([0.1, 0.1, 0.0], [-0.1, 0.1, 0.0], [0.0, 0.0, 0.0]),
                [1.0, 0.0, 0.0],
            )))
            .build(),
    )
    .unwrap();
    app.run().unwrap();
}
