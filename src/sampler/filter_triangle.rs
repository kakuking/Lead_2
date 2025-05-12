use crate::common::*;

#[derive(Debug, Clone)]
pub struct TriangleFilter {
    radius: Vector2,
    inv_radius: Vector2
}

impl TriangleFilter {
    pub fn init(radius: Vector2) -> Self {
        let mut ret = TriangleFilter {
            radius: Vector2::new(0.0, 0.0),
            inv_radius: Vector2::new(0.0, 0.0)
        };

        Filter::initialize(&mut ret, radius);
        ret
    }
}

impl Filter for TriangleFilter {
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

    fn evaluate(&self, p: &Point2) -> Float {
        (self.radius.x - p.x.abs()).max(0.0) * (self.radius.y - p.y.abs()).max(0.0)
    }
}