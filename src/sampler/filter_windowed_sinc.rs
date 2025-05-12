use crate::common::*;

#[derive(Debug, Clone)]
pub struct WindowedSincFilter {
    radius: Vector2,
    inv_radius: Vector2,
    tau: Float
}

impl WindowedSincFilter {
    pub fn init(radius: Vector2, tau: Float) -> Self {
        let mut ret = WindowedSincFilter {
            radius: Vector2::new(0.0, 0.0),
            inv_radius: Vector2::new(0.0, 0.0),
            tau
        };

        Filter::initialize(&mut ret, radius);
        ret
    }

    fn sinc(x: Float) -> Float {
        let x = x.abs();
        if x < EPSILON { return 1.0; }

        (PI * x).sin() / (PI * x)
    }

    fn windowed_sinc(&self, x: Float, radius: Float) -> Float {
        let x = x.abs();
        if x > radius { return 0.0; }
        let lanczos = Self::sinc(x / self.tau);

        Self::sinc(x) * lanczos
    }
}

impl Filter for WindowedSincFilter {
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
        self.windowed_sinc(p.x, self.radius.x) * self.windowed_sinc(p.y, self.radius.y)
    }
}