use crate::common::*;

#[derive(Debug, Clone)]
pub struct BoxFilter {
    radius: Vector2,
    inv_radius: Vector2
}

impl BoxFilter {
    pub fn init(radius: Vector2) -> Self {
        let mut ret = BoxFilter {
            radius: Vector2::new(0.0, 0.0),
            inv_radius: Vector2::new(0.0, 0.0)
        };

        Filter::initialize(&mut ret, radius);
        ret
    }
}

impl Filter for BoxFilter {
    fn radius(&self) -> Vector2 {
        self.radius
    }

    fn inv_radius(&self) -> Vector2 {
        self.inv_radius
    }

    fn set_radius(&mut self, other: Vector2) {
        self.radius = other;
    }

    fn set_inv_radius(&mut self, other: Vector2) {
        self.inv_radius = other;
    }

    fn evaluate(&self, _p: &Point2) -> Float {
        1.0
    }
}