use crate::common::*;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Ray {
    pub o: na::Point3<Float>,
    pub d: na::Vector3<Float>,
    pub t_max: Float,
    pub time: Float,
    pub medium: Option<Arc<dyn Medium>>
}

impl Ray {
    pub fn new() -> Self {
        Self {
            t_max: INFINITY,
            time: 0.0,
            o: na::Point3::new(0.0, 0.0, 0.0),
            d: na::Vector3::new(0.0, 0.0, 0.0),
            medium: None
        }
    }

    pub fn init(o: &Point3, d: &Vector3, t_max: Option<Float>, time: Option<Float>, medium: Option<Arc<dyn Medium>>) -> Self {
        Self {
            o: o.clone(),
            d: d.clone(),
            t_max: match t_max {
                Some(t) => t,
                None => INFINITY
            },
            time: match time {
                Some(t) => t,
                None => 0.0
            },
            medium: medium
        }
    }

    pub fn at(&self, t: Float) -> Point3 {
        return self.o + self.d * t;
    }

    pub fn to_string(&self) -> String {
        format!(
            "Origin: {}, Direction: {}, t_max: {}, time: {}",
            self.o, self.d, self.t_max, self.time
        )
    }
}

#[derive(Debug, Clone)]
pub struct RayDifferential {
    pub ray: Ray,
    pub has_differentials: bool,
    pub rx_origin: Point3,
    pub ry_origin: Point3,
    pub rx_direction: Vector3,
    pub ry_direction: Vector3
}

impl RayDifferential {
    pub fn new() -> Self {
        Self {
            ray: Ray::new(),
            has_differentials: false,
            rx_origin: Point3::new(0.0, 0.0, 0.0),
            ry_origin: Point3::new(0.0, 0.0, 0.0),
            rx_direction: Vector3::new(0.0, 0.0, 0.0),
            ry_direction: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn init(o: &Point3, d: &Vector3, t_max: Option<Float>, time: Option<Float>, medium: Option<Arc<dyn Medium>>) -> Self {
        Self {
            ray: Ray::init(o, d, t_max, time, medium),
            has_differentials: false,
            rx_origin: Point3::new(0.0, 0.0, 0.0),
            ry_origin: Point3::new(0.0, 0.0, 0.0),
            rx_direction: Vector3::new(0.0, 0.0, 0.0),
            ry_direction: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn init_ray(r: &Ray) -> Self {
        Self {
            ray: r.clone(),
            has_differentials: false,
            rx_origin: Point3::new(0.0, 0.0, 0.0),
            ry_origin: Point3::new(0.0, 0.0, 0.0),
            rx_direction: Vector3::new(0.0, 0.0, 0.0),
            ry_direction: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn scale_differentials(mut self, s: Float) {
        self.rx_origin = self.ray.o + (&self.rx_origin - self.ray.o) * s;
        self.ry_origin = self.ray.o + (&self.ry_origin - self.ray.o) * s;
        self.rx_direction = self.ray.d + (&self.rx_direction - self.ray.d) * s;
        self.ry_direction = self.ray.d + (&self.ry_direction - self.ray.d) * s;
    }
}

pub fn offset_ray_origin(p: &Point3, p_error: &Vector3, n: &Vector3, wo: &Vector3) -> Point3 {
    let d = n.abs().dot(p_error);
    let mut offset = d * Vector3::new(d, d, d);

    if n.dot(wo) < 0.0 {
        offset = -offset;
    }

    let mut po = p + offset;

    for i in 0..3 {
        if offset[i] > 0.0 {
            po[i] = next_float_up(po[i]);
        } else if offset[i] < 0.0 {
            po[i] = next_float_down(po[i]);
        }
    }

    po
}

impl Mul<&Ray> for Transform {
    type Output = Ray;

    fn mul(self, rhs: &Ray) -> Self::Output {
        let o_new = self * rhs.o;
        let d_new = self * rhs.d;

        Ray::init(&o_new, &d_new, Some(rhs.t_max), Some(rhs.time), rhs.medium.clone())
    }
}

impl Mul<&RayDifferential> for Transform {
    type Output = RayDifferential;

    fn mul(self, rhs: &RayDifferential) -> Self::Output {
        let ray = self * &rhs.ray;
        let rx_origin = self * rhs.rx_origin;
        let rx_direction = self * rhs.rx_direction;
        let ry_origin = self * rhs.ry_origin;
        let ry_direction = self * rhs.ry_direction;

        RayDifferential {
            ray,
            rx_origin, rx_direction,
            ry_origin, ry_direction,
            has_differentials: rhs.has_differentials
        }
    }
}