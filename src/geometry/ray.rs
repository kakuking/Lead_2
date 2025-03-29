use crate::common::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub o: na::Point3<Float>,
    pub d: na::Vector3<Float>,
    pub t_max: Float,
    pub time: Float,
    // pub medium: Arc<Medium>
}

impl Ray {
    pub fn new() -> Self {
        Self {
            t_max: INFINITY,
            time: 0.0,
            o: na::Point3::new(0.0, 0.0, 0.0),
            d: na::Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn init(o: &Point3, d: &Vector3, t_max: Option<Float>, time: Option<Float>) -> Self {
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
            }
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

#[derive(Debug, Clone, Copy)]
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

    pub fn init(o: &Point3, d: &Vector3, t_max: Option<Float>, time: Option<Float>) -> Self {
        Self {
            ray: Ray::init(o, d, t_max, time),
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