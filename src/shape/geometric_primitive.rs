pub use crate::common::*;

#[derive(Debug, Clone)]
pub struct GeometricPrimitive {
    shape: Arc<dyn Shape>,
    material: Option<Arc<dyn Material>>,
    area_light: Option<Arc<dyn AreaLight>>,
    medium_interface: Option<MediumInterface>,
}

impl GeometricPrimitive {
    pub fn init(shape: Arc<dyn Shape>, material: Option<Arc<dyn Material>>, area_light: Option<Arc<dyn AreaLight>>, medium_interface: Option<MediumInterface>) -> Self {
        Self {
            shape,
            material,
            area_light,
            medium_interface
        }
    }
}

impl Primitive for GeometricPrimitive {
    fn world_bound(&self) -> Bounds3f {
        self.shape.world_bound()
    }

    fn get_area_light(&self) -> Option<Arc<dyn AreaLight>> {
        self.area_light.clone()
    }

    fn get_material(&self) -> Option<Arc<dyn Material>> {
        self.material.clone()
    }

    fn intersect(&self, ray: &mut Ray, isect: &mut SurfaceInteraction) -> bool {
        let mut t_hit: Float = 0.0;
        if !self.shape.intersect(ray, &mut t_hit, isect, false) {
            return false;
        }

        (*ray).t_max = t_hit;
        (*isect).shape = Some(self.shape.clone());

        // set medium of isect
        if let Some(mi) = &self.medium_interface {
            if mi.is_medium_transition()  {
                (*isect).interaction.medium_interface = self.medium_interface.clone();
            } else {
                if let Some(mi) = &ray.medium {
                    (*isect).interaction.medium_interface = Some(MediumInterface::init_one(mi.clone()));
                } else {
                    (*isect).interaction.medium_interface = None;
                }
            }
        }

        true
    }

    fn intersect_p(&self, r: &mut Ray) -> bool {
        self.shape.intersect_p(r, false)
    }

    fn compute_scattering_function(&self, isect: &mut SurfaceInteraction, mode: TransportMode, allow_multiple_lobes: bool) {
        if let Some(material) = &self.material {
            material.compute_scattering_function(isect, mode, allow_multiple_lobes);
        }
    }
}