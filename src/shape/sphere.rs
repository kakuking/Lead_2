use crate::common::*;

#[derive(Debug, Clone)]
pub struct Sphere {
    object_to_world: Arc<Transform>,
    world_to_object: Arc<Transform>,
    reverse_orientation: bool,
    transform_swaps_handedness: bool,

    radius: Float,
    z_min: Float, z_max: Float,
    theta_min: Float, theta_max: Float,
    phi_max: Float,
}

impl Sphere {
    pub fn init(object_to_world: Arc<Transform>, world_to_object: Arc<Transform>, reverse_orientation: bool, radius: Float, z_min: Float, z_max: Float, phi_max: Float) -> Self {
        let z_min = z_min.min(z_max).clamp(-radius, radius);
        let z_max = z_max.max(z_min).clamp(-radius, radius);
        let theta_min = (z_min / radius).clamp(-1.0, 1.0).acos();
        let theta_max = (z_max / radius).clamp(-1.0, 1.0).acos();
        let phi_max = phi_max.clamp(0.0, 360.0).to_radians();

        let tsh = arc_transform_swaps_handedness(object_to_world.clone());
        
        Self {
            object_to_world: object_to_world,
            world_to_object: world_to_object,
            reverse_orientation: reverse_orientation,
            transform_swaps_handedness: tsh,

            radius: radius,
            z_min, z_max,
            theta_min, theta_max,
            phi_max,
        }
    }
}

impl Shape for Sphere {
    fn object_to_world(&self) -> Arc<Transform> { self.object_to_world.clone() }
    fn world_to_object(&self) -> Arc<Transform> { self.world_to_object.clone() }
    fn reverse_orientation(&self) -> bool { self.reverse_orientation }
    fn transform_swaps_handedness(&self) -> bool { self.transform_swaps_handedness }

    fn set_object_to_world(&mut self, t: Arc<Transform>) { self.object_to_world = t; }
    fn set_world_to_object(&mut self, t: Arc<Transform>) { self.world_to_object = t; }
    fn set_reverse_orientation(&mut self, t: bool) { self.reverse_orientation = t; }
    fn set_transform_swaps_handedness(&mut self, t: bool) { self.transform_swaps_handedness = t; }

    fn area(&self) -> Float {
        self.phi_max * self.radius * (self.z_max - self.z_min)
    }

    fn object_bound(&self) -> Bounds3f {
        let p_min = Point3::new(-self.radius, -self.radius, -self.radius);
        let p_max = -p_min;

        Bounds3f::init(&p_min, &p_max)
    }

    fn world_bound(&self) -> Bounds3f {
        *self.object_to_world() * &self.object_bound()
    }

    fn intersect(&self, ray: &Ray, t_hit: &mut Float, isect: &mut SurfaceInteraction, _test_alpha_texture: bool) -> bool {
        let mut phi: Float;
        let mut p_hit: Point3;

        let ray = *self.world_to_object * ray;

        let a = ray.d.x * ray.d.x + ray.d.y * ray.d.y + ray.d.z * ray.d.z;
        let b = 2.0 * (ray.d.x * ray.o.x + ray.d.y * ray.o.y + ray.d.z * ray.o.z);
        let c = ray.o.x * ray.o.x + ray.o.y * ray.o.y + ray.o.z * ray.o.z;

        let mut t0: Float = 0.0;
        let mut t1: Float = 0.0;
        if !quadratic(a, b, c, &mut t0, &mut t1) {
            return false;
        }

        // Degen case
        if t0 > ray.t_max || t1 <= 0.0 {
            return false;
        }

        let mut t_shape_hit = t0;
        if t_shape_hit <= 0.0 {
            t_shape_hit = t1;
            if t_shape_hit > ray.t_max {
                return false;
            }
        }

        p_hit = ray.at(t_shape_hit);
        p_hit *= self.radius / (p_hit - Point3::new(0.0, 0.0, 0.0)).norm();

        if p_hit.x == 0.0 && p_hit.y == 0.0 { p_hit.x = EPSILON * self.radius; }// if x and u are 0 0, then shift x a bit

        phi = p_hit.y.atan2(p_hit.x);
        if phi < 0.0 { phi += 2.0 * PI; }

        // Check against zminmax and phimax
        if (self.z_min > -self.radius && p_hit.z < self.z_min) || (self.z_max < self.radius && p_hit.z > self.z_max) || phi > self.phi_max {
            if t_shape_hit == t1 { return false; }  // If its the second hit, return false
            if t1 > ray.t_max { return false; }
            t_shape_hit = t1;

            p_hit = ray.at(t_shape_hit);
            p_hit *= self.radius / (p_hit - Point3::new(0.0, 0.0, 0.0)).norm();

            if p_hit.x == 0.0 && p_hit.y == 0.0 { p_hit.x = EPSILON * self.radius; }    // if x and u are 0 0, then shift x a bit 

            phi = p_hit.y.atan2(p_hit.x);
            if phi < 0.0 { phi += 2.0 * PI; }

            // if still out of bounds return false
            if (self.z_min > -self.radius && p_hit.z < self.z_min) || (self.z_max < self.radius && p_hit.z > self.z_max) || phi > self.phi_max { return false}
        }

        let u = phi / self.phi_max;
        let theta = (p_hit.z / self.radius).clamp(-1.0, 1.0).acos();
        let v = (theta - self.theta_min) / (self.theta_max - self.theta_min);

        let z_radius = (p_hit.x * p_hit.x + p_hit.y * p_hit.y).sqrt();
        let inv_z_radius = 1.0 / z_radius;
        let cos_phi = p_hit.x * inv_z_radius;
        let sin_phi = p_hit.y * inv_z_radius;

        let dpdu = Vector3::new(-self.phi_max * p_hit.y, self.phi_max * p_hit.x, 0.0);
        let dpdv = (self.theta_max - self.theta_min) * Vector3::new(p_hit.z * cos_phi, p_hit.z * sin_phi, -self.radius * theta.sin());

        let d2pduu = -self.phi_max * self.phi_max * Vector3::new(p_hit.x, p_hit.y, 0.0);
        let d2pduv = (self.theta_max - self.theta_min) * p_hit.z * self.phi_max * Vector3::new(-sin_phi, cos_phi, 0.0);
        let d2pdvv = -(self.theta_max - self.theta_min) * (self.theta_max - self.theta_min) * Vector3::new(p_hit.x, p_hit.y, p_hit.z);

        let big_e = dpdu.dot(&dpdu);
        let big_f = dpdu.dot(&dpdv);
        let big_g = dpdv.dot(&dpdv);

        let n = dpdu.cross(&dpdv).normalize();

        let e = n.dot(&d2pduu);
        let f = n.dot(&d2pduv);
        let g = n.dot(&d2pdvv);

        let inv_efg2 = 1.0 / (big_e * big_g - big_f * big_f);
        let dndu = (f*big_f - e*big_g)*inv_efg2*dpdu + (e*big_f - f*big_e)*inv_efg2*dpdv;
        let dndv = (g*big_f - f*big_g)*inv_efg2*dpdu + (f*big_f - g*big_e)*inv_efg2*dpdv;

        let p_error = gamma(5.0) * p_hit.map(|e| e.abs()) - Point3::new(0.0, 0.0, 0.0);

        *isect = SurfaceInteraction::init(&p_hit, &p_error, &Point2::new(u, v), &(-ray.d), &dpdu, &dpdv, &dndu, &dndv, ray.time, None);
        *t_hit = t_shape_hit;

        true
    }

    fn intersect_p(&self, ray: &Ray, test_alpha_texture: bool) -> bool {
        let mut t = 0.0;
        let mut isect = SurfaceInteraction::new();

        self.intersect(ray, &mut t, &mut isect, test_alpha_texture)
    }

    fn pdf(&self, _: &Interaction) -> Float {
        // TODO
        1.0 / self.area()
    }

    fn pdf_ref(&self, _reference: &Interaction, _wi: &Vector3) -> Float {
        // TODO
        1.0 / self.area()
    }

    fn sample(&self, _u: &Point2) -> Interaction {
        // TODO
        Interaction::new()
    }
    
    fn sample_ref(&self, _reference: &Interaction, _u: &Point2) -> Interaction {
        // TODO
        Interaction::new()
    }
}