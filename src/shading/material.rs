

use crate::common::*;

pub trait Material: Debug {
    fn compute_scattering_function(&self, isect: &mut SurfaceInteraction, mode: TransportMode, allow_multiple_lobes: bool);
}