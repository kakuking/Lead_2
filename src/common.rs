pub use nalgebra as na;
pub use std::sync::Arc;
pub use std::fmt::Debug;

pub use crate::geometry::*;
pub use crate::medium::*;
pub use crate::interaction::*;
pub use crate::shape::*;
pub use crate::shading::*;
pub use crate::spectrum::*;
pub use crate::camera::*;
pub use crate::math::*;

// can set it between f32 and f64 here, just like pbr-book does
pub type Float = f32;
pub type Transform = na::Projective3<Float>;
pub type Spectrum = RBGSpectrum;

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

pub fn scale(scale: &Vector3) -> Transform {
    // na::Projective3::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), scale)
    let scale_matrix = na::Matrix4::new_nonuniform_scaling(scale);

    Transform::from_matrix_unchecked(scale_matrix)
}

pub fn translate(translate: &Vector3) -> Transform {
    let matrix = na::Matrix4::new_translation(translate);

    Transform::from_matrix_unchecked(matrix)
}

pub fn rotate(axis_angle: Vector3) -> Transform {
    let matrix = na::Matrix4::new_rotation(axis_angle);

    Transform::from_matrix_unchecked(matrix)
}


pub fn sample_concentric_disc(u: &Point2) -> Point2 {
    let u_offset = 2.0 * u - Vector2::new(1.0, 1.0);

    if u_offset.x == 0.0 && u_offset.y == 0.0 {
        return Point2::new(0.0, 0.0);
    }

    let theta: Float;
    let r: Float;

    if u_offset.x.abs() > u_offset.y.abs() {
        r = u_offset.x;
        theta = PI / 4.0 * (u_offset.y / u_offset.x);
    } else {
        r = u_offset.y;
        theta = PI/2.0 - u_offset.x / u_offset.y * PI / 4.0;
    }

    r * Point2::new(theta.cos(), theta.sin())
}