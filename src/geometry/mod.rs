pub mod ray;
pub mod bounding_box_2;
pub mod bounding_box_3;

pub use ray::{Ray, RayDifferential, offset_ray_origin};
pub use bounding_box_2::Bounds2f;
pub use bounding_box_3::Bounds3f;

use crate::common::{Vector3, Transform, Arc};

pub fn face_forward(n: &Vector3, v: &Vector3) -> Vector3 {
    return if n.dot(v) < 0.0 {
        -n
    } else {
        n.clone()
    }
}

pub fn apply_transform_to_normal(n: &Vector3, t: &Transform) -> Vector3 {
    let lin = t.isometry.rotation.to_rotation_matrix();
    let mat = lin.inverse().transpose();

    mat * n
}

pub fn transform_swaps_handedness(t: &Transform) -> bool {
    t.isometry.rotation.to_rotation_matrix().matrix().determinant() * t.scaling().powi(3) < 0.0
}

pub fn arc_transform_swaps_handedness(t: Arc<Transform>) -> bool {
    t.isometry.rotation.to_rotation_matrix().matrix().determinant() * t.scaling().powi(3) < 0.0
}