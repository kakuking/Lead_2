

use crate::common::*;

pub trait Primitive: Debug {
    fn world_bound(&self) -> Bounds3f;
    // here set the shape value in the SurfaceInteraction as Some(Arc<dyn Shape>)
    fn intersect(&self, ray: &mut Ray, isect: &mut SurfaceInteraction) -> bool;
    fn intersect_p(&self, ray: &mut Ray) -> bool;
    fn get_area_light(&self) -> Option<Arc<dyn AreaLight>>;
    fn get_material(&self) -> Option<Arc<dyn Material>>;
    fn compute_scattering_function(&self, isect: &mut SurfaceInteraction, mode: TransportMode, allow_multiple_lobes: bool);
}