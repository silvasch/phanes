use phanes::renderer::Renderer;

fn main() {
    let renderer = pollster::block_on(Renderer::new().build()).unwrap();
    renderer.run().unwrap();
}
