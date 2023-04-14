use phanes::Phanes;

fn main() {
    let app = pollster::block_on(Phanes::new().build()).unwrap();
    app.run().unwrap();
}
