use crate::{rendering_engine::RenderingEngine, PhanesError};

pub struct Phanes {
    rendering_engine: RenderingEngine,
}

impl Phanes {
    pub fn new() -> PhanesBuilder {
        PhanesBuilder::new()
    }

    pub fn run(self) -> Result<(), PhanesError> {
        Ok(())
    }
}

pub struct PhanesBuilder {}

impl PhanesBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self) -> Phanes {
        Phanes {
            rendering_engine: RenderingEngine::new(),
        }
    }
}
