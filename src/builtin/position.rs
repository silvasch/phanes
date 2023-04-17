pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn to_screen_space_coords(&self, size: winit::dpi::PhysicalSize<u32>) -> (f32, f32) {
        (
            self.x / size.width as f32 * 2 as f32 - 1 as f32,
            self.y / size.height as f32 * 2 as f32 - 1 as f32,
        )
    }
}
