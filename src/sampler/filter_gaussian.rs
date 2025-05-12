use crate::common::*;

#[derive(Debug, Clone)]
pub struct GaussianFilter {
    radius: Vector2,
    inv_radius: Vector2,

    alpha: Float,
    exp_x: Float,
    exp_y: Float
}

impl GaussianFilter {
    pub fn init(radius: Vector2, alpha: Float) -> Self {
        let mut ret = GaussianFilter {
            radius: Vector2::new(0.0, 0.0),
            inv_radius: Vector2::new(0.0, 0.0),
            alpha: alpha,
            exp_x: (-alpha * radius.x * radius.x).exp(),
            exp_y: (-alpha * radius.y * radius.y).exp(),
        };

        Filter::initialize(&mut ret, radius);
        ret
    }

    fn gaussian(&self, d: Float, exp_v: Float) -> Float {
        ((-self.alpha * d * d).exp() - exp_v).max(0.0)
    }
}

impl Filter for GaussianFilter {
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
        self.gaussian(p.x, self.exp_x) * self.gaussian(p.y, self.exp_y)
    }
}