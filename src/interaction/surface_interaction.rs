use std::ops::Mul;

use crate::common::*;

#[derive(Debug, Clone, Copy)]
pub struct ShadingInfo {
    pub n: Vector3, 
    pub dpdu: Vector3, pub dpdv: Vector3,
    pub dndu: Vector3, pub dndv: Vector3
}

impl ShadingInfo {
    pub fn new() -> Self {
        Self {
            n: Vector3::new(0.0, 0.0, 0.0),
            dpdu: Vector3::new(0.0, 0.0, 0.0), dpdv: Vector3::new(0.0, 0.0, 0.0),
            dndu: Vector3::new(0.0, 0.0, 0.0), dndv: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn init(n: &Vector3, dpdu: &Vector3, dpdv: &Vector3, dndu: &Vector3, dndv: &Vector3) -> Self {
        Self {
            n: n.clone(),
            dpdu: dpdu.clone(), dpdv: dpdv.clone(),
            dndu: dndu.clone(), dndv: dndv.clone() 
        }
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceInteraction {
    pub interaction: Interaction,
    pub uv: Point2,

    pub dpdu: Vector3, pub dpdv: Vector3,
    pub dndu: Vector3, pub dndv: Vector3,
    
    pub shading: ShadingInfo,

    pub shape: Option<Arc<dyn Shape>>,
    pub primitive: Option<Arc<dyn Primitive>>,
    pub bsdf: Option<Arc<dyn BSDF>>,
    pub bssrdf: Option<Arc<dyn BSDF>>,
    
    pub dpdx: Vector3, pub dpdy: Vector3,
    pub dudx: Float, pub dvdx: Float,
    pub dudy: Float, pub dvdy: Float
}

impl SurfaceInteraction {
    pub fn new() -> Self {
        Self {
            interaction: Interaction::new(),
            uv: Point2::new(0.0, 0.0),

            dpdu: Vector3::new(0.0, 0.0, 0.0), dpdv: Vector3::new(0.0, 0.0, 0.0),
            dndu: Vector3::new(0.0, 0.0, 0.0), dndv: Vector3::new(0.0, 0.0, 0.0),

            shading: ShadingInfo::new(),

            shape: None,
            primitive: None,
            bsdf: None, bssrdf: None,

            dpdx: Vector3::new(0.0, 0.0, 0.0), dpdy: Vector3::new(0.0, 0.0, 0.0),
            dudx: 0.0, dvdx: 0.0,
            dudy: 0.0, dvdy: 0.0
        }
    }

    pub fn init(p: &Point3, p_error: &Vector3, uv: &Point2, wo: &Vector3, 
        dpdu: &Vector3, dpdv: &Vector3, dndu: &Vector3, dndv: &Vector3, time: Float, shape: Option<Arc<dyn Shape>>) -> Self {
        let mut normal = dpdu.cross(dpdv).normalize();
        let its = Interaction::init(p, wo, &normal, p_error, time, None);
        let mut shading = ShadingInfo {
            n: normal,
            dpdu: dpdu.clone(), dpdv: dpdv.clone(),
            dndu: dndu.clone(), dndv: dndv.clone()
        };

        if let Some(shape) = &shape {
            if shape.reverse_orientation() ^ shape.transform_swaps_handedness() {
                normal *= -1.0;
                shading.n *= -1.0;
            }
        }

        Self {
            interaction: its,
            uv: uv.clone(),

            dpdu: dpdu.clone(), dpdv: dpdv.clone(),
            dndu: dndu.clone(), dndv: dndv.clone(),

            shading,

            shape,
            primitive: None,
            bsdf: None, bssrdf: None,

            dpdx: Vector3::new(0.0, 0.0, 0.0), dpdy: Vector3::new(0.0, 0.0, 0.0),
            dudx: 0.0, dvdx: 0.0,
            dudy: 0.0, dvdy: 0.0
        }
    }

    pub fn set_shading_geometry(&mut self, dpdu: &Vector3, dpdv: &Vector3, dndu: &Vector3, dndv: &Vector3, orientation_is_authoritative: bool) {
        self.shading.n = dpdu.cross(dpdv).normalize();

        if let Some(shape) = &self.shape {
            if shape.reverse_orientation() ^ shape.transform_swaps_handedness() {
                self.shading.n *= 1.0;
            }
        }

        if orientation_is_authoritative {
            self.interaction.n = face_forward(&self.interaction.n, &self.shading.n)
        } else {
            self.shading.n = face_forward(&self.shading.n, &self.interaction.n);
        }

        self.shading.dpdu = dpdu.clone();
        self.shading.dpdv = dpdv.clone();
        self.shading.dndu = dndu.clone();
        self.shading.dndv = dndv.clone();
    }

    pub fn compute_scattering_function(&mut self, _ray: &RayDifferential, _allow_multiple_lobes: bool, _mode: TransportMode) {
        // TODO
    }

    pub fn compute_differential(&self, _r: &RayDifferential) {
        // TODO
    }

    pub fn le(&self, _w: &Vector3) {
        // TODO
    }
}

impl Mul<&SurfaceInteraction> for Transform {
    type Output = SurfaceInteraction;

    fn mul(self, rhs: &SurfaceInteraction) -> Self::Output {
        let arc_self = Arc::from(self);

        let p = self * rhs.interaction.p;
        let p_error = self * rhs.interaction.p_error;
        let n = self * rhs.interaction.n;
        let wo =  self * rhs.interaction.wo;
        let time = rhs.interaction.time;
        let mi = rhs.interaction.medium_interface.clone();

        let uv = rhs.uv;

        let shape = rhs.shape.clone();
        let dpdu = self * rhs.dpdu;
        let dpdv = self * rhs.dpdv;
        let dndu = apply_transform_to_normal(&rhs.dndu, &arc_self);
        let dndv = apply_transform_to_normal(&rhs.dndv, &arc_self);

        let mut sha_n =  apply_transform_to_normal(&rhs.shading.n, &arc_self);
        let sha_dpdu = self * rhs.shading.dpdu;
        let sha_dpdv = self * rhs.shading.dpdv;
        let sha_dndu = apply_transform_to_normal(&rhs.shading.dndu, &arc_self);
        let sha_dndv = apply_transform_to_normal(&rhs.shading.dndv, &arc_self);

        let dudx = rhs.dudx; let dvdx = rhs.dvdx; let dudy = rhs.dudy; let dvdy = rhs.dvdy;

        let dpdx = self * rhs.dpdx;
        let dpdy = self * rhs.dpdy;

        sha_n = face_forward(&sha_n, &n);

        let mut ret = SurfaceInteraction::init(&p, &p_error, &uv, &wo, &dpdu, &dpdv, &dndu, &dndv, time, shape);

        ret.interaction.medium_interface = mi;
        ret.shading.n = sha_n;
        ret.shading.dpdu = sha_dpdu;
        ret.shading.dpdv = sha_dpdv;
        ret.shading.dndu = sha_dndu;
        ret.shading.dndv = sha_dndv;

        ret.dudx = dudx;
        ret.dvdx = dvdx;
        ret.dudy = dudy;
        ret.dvdy = dvdy;

        ret.dpdx = dpdx;
        ret.dpdy = dpdy;

        ret
    }
}