pub use crate::common::*;

#[derive(Debug)]
pub struct BruteForceAggregate{
    prims: Vec<Arc<dyn Primitive>>,
    bounds: Bounds3f
}

impl BruteForceAggregate{
    pub fn new() -> Self {
        Self {
            prims: Vec::new(),
            bounds: Bounds3f::new()
        }
    }

    pub fn add_primitive(&mut self, prim: Arc<dyn Primitive>){
        self.bounds = Bounds3f::union(&self.bounds, &prim.world_bound());
        self.prims.push(prim);
    }
}

impl Primitive for BruteForceAggregate {
    fn compute_scattering_function(&self, _isect: &mut SurfaceInteraction, _mode: TransportMode, _allow_multiple_lobes: bool) {
        
    }

    fn get_area_light(&self) -> Option<Arc<dyn AreaLight>> {
        panic!("Do not use");
    }

    fn get_material(&self) -> Option<Arc<dyn Material>> {
        panic!("Do not use");
    }

    fn world_bound(&self) -> Bounds3f {
        self.bounds
    }

    fn intersect(&self, ray: &mut Ray, isect: &mut SurfaceInteraction) -> bool {
        let mut hit = false;
        let mut cur_t = INFINITY;
        
        for i in 0..self.prims.len(){
            let mut its = SurfaceInteraction::new();
            let prim = &self.prims[i];

            if prim.intersect(ray, &mut its) {
                hit = true;
                if its.interaction.time < cur_t {
                    cur_t = its.interaction.time;
                    *isect = its;
                }
            }
        }

        hit
    }

    fn intersect_p(&self, ray: &mut Ray) -> bool {
        let mut temp = SurfaceInteraction::new();

        self.intersect(ray, &mut temp)
    }
}