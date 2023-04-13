use phanes::renderer::Renderer;

fn main() {
    env_logger::init();
    color_eyre::install().unwrap();

    let renderer = pollster::block_on(Renderer::new().build()).unwrap();

    renderer.run().unwrap();
}
