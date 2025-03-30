use std::fmt::Debug;
use crate::common::*;


pub trait Shape: Debug {
    fn object_to_world(&self) -> Arc<Transform>;
    fn world_to_object(&self) -> Arc<Transform>;
    fn reverse_orientation(&self) -> bool;
    fn transform_swaps_handedness(&self) -> bool;

    fn set_object_to_world(&mut self, t: Arc<Transform>);
    fn set_world_to_object(&mut self, t: Arc<Transform>);
    fn set_reverse_orientation(&mut self, t: bool);
    fn set_transform_swaps_handedness(&mut self, t: bool);

    fn set_transforms(&mut self, object_to_world: Arc<Transform>, world_to_object: Arc<Transform>, reverse_orientation: bool) {
        self.set_object_to_world(object_to_world.clone());
        self.set_world_to_object(world_to_object);
        self.set_reverse_orientation(reverse_orientation);
        self.set_transform_swaps_handedness(arc_transform_swaps_handedness(object_to_world));
    }

    fn object_bound(&self) -> Bounds3f;
    fn world_bound(&self) -> Bounds3f;

    fn intersect(&self, ray: &Ray, t_hit: &mut Float, isect: &mut SurfaceInteraction, test_alpha_texture: bool) -> bool;
    fn intersect_p(&self, ray: &Ray, test_alpha_texture: bool) -> bool {
        let mut t_hit = ray.t_max;
        let mut isect = SurfaceInteraction::new();
        self.intersect(ray, &mut t_hit, &mut isect, test_alpha_texture)
    }
    
    fn area(&self) -> Float;
    fn sample(&self, u: &Point2) -> Interaction;
    fn pdf(&self, _: &Interaction) -> Float {
        1.0 / self.area()
    }

    fn sample_ref(&self, reference: &Interaction, u: &Point2) -> Interaction;
    fn pdf_ref(&self, reference: &Interaction, wi: &Vector3) -> Float;
}

pub trait Primitive: Debug {
    
} 