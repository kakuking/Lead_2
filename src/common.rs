pub use nalgebra as na;
pub use std::sync::Arc;

pub use crate::geometry::*;
pub use crate::medium::*;
pub use crate::interaction::*;
pub use crate::shape::*;
pub use crate::shading::*;

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

pub fn next_float_up(v: f32) -> f32 {
    // Handle infinity and negative zero
    if v.is_infinite() && v > 0.0 {
        return v;
    }
    if v == -0.0 {
        return 0.0;
    }

    // Advance v to next higher float by manipulating bits
    let bits = v.to_bits();
    let next_bits = if v >= 0.0 {
        bits + 1
    } else {
        bits - 1
    };
    f32::from_bits(next_bits)
}

/// Returns the next representable floating-point number less than `v`.
pub fn next_float_down(v: f32) -> f32 {
    -next_float_up(-v)
}

pub enum TransportMode {
    Radiance,
    Importance
}