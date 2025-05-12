use crate::common::*;

pub trait Filter: Debug {
    fn radius(&self) -> Vector2;
    fn inv_radius(&self) -> Vector2;

    fn set_radius(&mut self, other: Vector2);
    fn set_inv_radius(&mut self, other: Vector2);

    fn initialize(&mut self, radius: Vector2) {
        self.set_radius(radius);
        self.set_inv_radius(Vector2::new(1.0 / radius.x, 1.0 / radius.y));
    } 

    fn evaluate(&self, p: &Point2) -> Float;
}