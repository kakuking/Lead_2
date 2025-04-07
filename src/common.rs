pub use nalgebra as na;
pub use std::sync::Arc;
pub use std::fmt::Debug;

pub use crate::geometry::*;
pub use crate::medium::*;
pub use crate::interaction::*;
pub use crate::shape::*;
pub use crate::shading::*;
pub use crate::math::*;

// can set it between f32 and f64 here, just like pbr-book does
pub type Float = f32;
pub type Transform = na::Similarity3<Float>;

pub const INFINITY: Float = Float::INFINITY;
pub const EPSILON: Float = 0.0001;

pub type Point2 = na::Point2<Float>;
pub type Point3 = na::Point3<Float>;
pub type Vector2 = na::Vector2<Float>;
pub type Vector3 = na::Vector3<Float>;

pub enum TransportMode {
    Radiance,
    Importance
}