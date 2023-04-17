use crate::PhanesError;

pub struct Phanes {}

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
        Phanes {}
    }
}
