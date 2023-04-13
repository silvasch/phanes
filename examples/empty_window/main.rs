use phanes::app::App;

fn main() {
    let app = pollster::block_on(App::new().build()).unwrap();
    app.run().unwrap();
}
