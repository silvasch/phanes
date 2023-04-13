use phanes::app::App;

fn main() {
    env_logger::init();
    color_eyre::install().unwrap();

    let app = pollster::block_on(App::new().build()).unwrap();

    app.run().unwrap();
}
