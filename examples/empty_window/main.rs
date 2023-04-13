use phanes::renderer::Renderer;

fn main() {
    let renderer = Renderer::new()
        .build()
        .unwrap();
    renderer.run().unwrap();
}
