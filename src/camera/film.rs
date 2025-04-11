use crate::common::*;

#[derive(Debug)]
pub struct Film {
    pub full_resolution: Vector2
}

impl Film {
    pub fn new() -> Self {
        Self {
            full_resolution: Vector2::new(0.0, 0.0)
        }
    }
}