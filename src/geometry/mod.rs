pub mod ray;
pub mod bounding_box_2;
pub mod bounding_box_3;
pub mod helpers;

pub use ray::{Ray, RayDifferential, offset_ray_origin};
pub use bounding_box_2::Bounds2f;
pub use bounding_box_3::Bounds3f;
pub use helpers::{ceil, floor, min, max};

use crate::common::{Arc, Transform, Vector3};

pub fn face_forward(n: &Vector3, v: &Vector3) -> Vector3 {
    return if n.dot(v) < 0.0 {
        -n
    } else {
        n.clone()
    }
}

pub fn apply_transform_to_normal(n: &Vector3, t: &Arc<Transform>) -> Vector3 {
    // let lin = t.isometry.rotation.to_rotation_matrix();
    // let mat = lin.inverse().transpose();
    let mat3 = t.matrix().fixed_view::<3, 3>(0, 0).into_owned();
    let normal_matrix = mat3.try_inverse().unwrap().transpose(); 

    normal_matrix * n
}

pub fn transform_swaps_handedness(t: &Transform) -> bool {
    // t.isometry.rotation.to_rotation_matrix().matrix().determinant() * t.scaling().powi(3) < 0.0
    let lin = t.matrix().fixed_view::<3, 3>(0, 0);
    lin.determinant() < 0.0
}

pub fn arc_transform_swaps_handedness(t: Arc<Transform>) -> bool {
    // t.isometry.rotation.to_rotation_matrix().matrix().determinant() * t.scaling().powi(3) < 0.0
    let lin = t.matrix().fixed_view::<3, 3>(0, 0);
    lin.determinant() < 0.0
}

pub fn apply_transform_to_ray(r: &Ray, t: &Arc<Transform>) -> Ray {
    let r_o = t.transform_point(&r.o);
    let r_d = t.transform_vector(&r.d);

    Ray::init(&r_o, &r_d, Some(r.t_max), Some(r.time), r.medium.clone())
}

pub fn apply_transform_to_ray_differential(r: &RayDifferential, t: &Arc<Transform>) -> RayDifferential { 
    let ray = apply_transform_to_ray(&r.ray, t);

    let rx_origin = t.transform_point(&r.rx_origin); 
    let ry_origin = t.transform_point(&r.ry_origin); 
    let rx_direction = t.transform_vector(&r.rx_direction);
    let ry_direction = t.transform_vector(&r.ry_direction);
    let has_differentials = r.has_differentials;

    RayDifferential {
        ray,
        rx_origin,
        ry_origin,
        rx_direction,
        ry_direction,
        has_differentials
    }
}
