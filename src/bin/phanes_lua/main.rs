use phanes::Phanes;

fn main() {
    env_logger::init();
    color_eyre::install().unwrap();

    let app = pollster::block_on(Phanes::new().build()).unwrap();
    app.run().unwrap();
}
