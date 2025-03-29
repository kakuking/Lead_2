pub use nalgebra as na;

pub use crate::geometry::*;

// can set it between f32 and f64 here, just like pbr-book does
pub type Float = f32;

pub const INFINITY: Float = Float::INFINITY;
pub const EPSILON: Float = 0.0001;

pub type Point2 = na::Point2<Float>;
pub type Point3 = na::Point3<Float>;
pub type Vector2 = na::Vector2<Float>;
pub type Vector3 = na::Vector3<Float>;

pub fn lerp(t: Float, v1: Float, v2: Float) -> Float {
    (1.0 - t) * v1 + t * v2
}