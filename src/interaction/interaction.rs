use crate::common::*;

#[derive(Debug, Clone)]
pub struct Interaction {
    pub p: Point3,
    pub time: Float,
    pub p_error: Vector3,
    pub wo: Vector3,
    pub n: Vector3,
    pub medium_interface: Option<MediumInterface>
}

impl Interaction {
    pub fn new() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            time: 0.0,
            p_error: Vector3::new(0.0, 0.0, 0.0),
            wo: Vector3::new(0.0, 0.0, 0.0),
            n: Vector3::new(0.0, 0.0, 0.0),
            medium_interface: None
        }
    }

    pub fn init(p: &Point3, wo: &Vector3, n: &Vector3, p_error: &Vector3, time: Float, mi: Option<MediumInterface>) -> Self {
        Self {
            p: p.clone(),
            time,
            p_error: p_error.clone(),
            wo: wo.clone(),
            n: n.clone(),
            medium_interface: mi
        }
    }

    pub fn init_no_n(p: &Point3, wo: &Vector3, time: Float, mi: Option<MediumInterface>) -> Self {
        Self {
            p: p.clone(),
            time,
            p_error: Vector3::new(0.0, 0.0, 0.0),
            wo: wo.clone(),
            n: Vector3::new(0.0, 0.0,0.0),
            medium_interface: mi.clone()
        }
    }

    pub fn init_minimal(p: &Point3, time: Float, mi: Option<MediumInterface>) -> Self {
        Self {
            p: p.clone(),
            time,
            p_error: Vector3::new(0.0, 0.0, 0.0),
            wo: Vector3::new(0.0, 0.0, 0.0),
            n: Vector3::new(0.0, 0.0,0.0),
            medium_interface: mi
        }
    }

    pub fn is_surface_interaction(&self) -> bool {
        self.n != Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn is_medium_interaction(&self) -> bool {
        !self.is_surface_interaction()
    }

    pub fn spawn_ray(&self, d: &Vector3) -> Ray {
        let o = offset_ray_origin(&self.p, &self.p_error, &self.n, &self.wo);
        Ray::init(&o, d, Some(INFINITY), Some(self.time))
    }

    pub fn spawn_ray_to(&self, p2: &Point3) -> Ray {
        let o = offset_ray_origin(&self.p, &self.p_error, &self.n, &self.wo);
        let d= p2 - o;
        Ray::init(&o, &d, Some(1.0 - EPSILON), Some(self.time))
    }

    pub fn spawn_ray_to_intersection(&self, it: &Self) -> Ray {
        let o = offset_ray_origin(&self.p, &self.p_error, &self.n, &self.wo);
        let p = offset_ray_origin(&it.p, &it.p_error, &it.n, &it.wo);
        let d = p - o;

        Ray::init(&o, &d, Some(1.0 - EPSILON), Some(self.time))
    }

    pub fn get_medium(&self, w: &Vector3) -> Option<Arc<Medium>> {
        if let Some(mi) = &self.medium_interface {
            return if w.dot(&self.n) > 0.0 { 
                mi.outside.clone()
            } else {
                mi.inside.clone()
            };
        }

        None
    }

    // If inside == outside then return, else err
    pub fn get_medium_no_inter(&self) -> Option<Arc<Medium>> {
        if let Some(mi) = &self.medium_interface {
            assert!(mi.inside_outside_same());
            return mi.inside.clone(); 
        }
        None
    }
}