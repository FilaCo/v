use crate::app::AppResult;
use yadi::prelude::v0::*;

pub struct Kernel {}

impl Kernel {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&mut self) -> AppResult<()> {
        Ok(())
    }

    async fn init(&mut self) -> AppResult<()> {
        Ok(())
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}
