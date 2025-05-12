use crate::common::*;

#[derive(Debug, Clone)]
pub struct MitchellFilter {
    radius: Vector2,
    inv_radius: Vector2,
    b: Float,
    c: Float
}

impl MitchellFilter {
    pub fn init(radius: Vector2, b: Float, c: Float) -> Self {
        let mut ret = MitchellFilter {
            radius: Vector2::new(0.0, 0.0),
            inv_radius: Vector2::new(0.0, 0.0),
            b,
            c
        };

        Filter::initialize(&mut ret, radius);
        ret
    }

    fn mitchell_1d(&self, x: Float) -> Float {
        let x = (2.0 * x).abs();
        let b = self.b;
        let c = self.c;

        if x > 1.0 {
            return ((-b - 6.0*c) * x*x*x + (6.0*b + 30.0*c) * x*x + (-12.0*b - 48.0*c) * x + (8.0*b + 24.0*c)) * (1.0/6.0);
        }
        return ((12.0 - 9.0*b - 6.0*c) * x*x*x + (-18.0 + 12.0*b + 6.0*c) * x*x + (6.0 - 2.0*b)) * (1.0/6.0);
    }
}

impl Filter for MitchellFilter {
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
        self.mitchell_1d(p.x * self.inv_radius.x) * self.mitchell_1d(p.y * self.inv_radius.y)
    }
}