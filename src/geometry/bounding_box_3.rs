use crate::common::*;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use std::cmp::PartialEq;

#[derive(Debug, Clone, Copy)]
pub struct Bounds3f {
    pub p_min: Point3,
    pub p_max: Point3
}

impl Bounds3f {
    pub fn new() -> Self {
        let min_num: Float = -INFINITY;
        let max_num: Float = INFINITY;

        Self {
            p_min: Point3::new(min_num, min_num, min_num),
            p_max: Point3::new(max_num, max_num, max_num)
        }
    }

    pub fn init(p1: &Point3, p2: &Point3) -> Self {
        Self{
            p_min: Point3::new(p1.x.min(p2.x), p1.y.min(p2.y), p1.z.min(p2.z)),
            p_max: Point3::new(p1.x.max(p2.x), p1.y.max(p2.y), p1.z.max(p2.z)),
        }
    }

    pub fn init_one(p: &Point3) -> Self {
        Self{
            p_min: p.clone(),
            p_max: p.clone()
        }
    }

    pub fn diagonal(self) -> Vector3 {
        self.p_max - self.p_min
    }

    pub fn corner(self, corner: usize) -> Point3 {
        let x_idx = corner & 1usize;
        let y_idx = if corner & 2usize == 0usize { 0 } else { 1 };
        let z_idx = if corner & 4usize == 0usize { 0 } else { 1 };

        Point3::new(self[x_idx].x, self[y_idx].y, self[z_idx].z)
    }

    pub fn surface_area(self) -> Float {
        let diag = self.diagonal();
        2.0 * (diag.x * diag.y + diag.x * diag.z + diag.z * diag.y)
    }

    pub fn volume(self) -> Float {
        let diag = self.diagonal();
        diag.x * diag.y * diag.z
    }

    // Which is longer, x, y or z
    pub fn max_extent(self) -> usize {
        let diag = self.diagonal();

        if diag.x > diag.y && diag.x > diag.z {
            return 0;
        } else if diag.y > diag.z {
            return 1;
        }
        2
    }

    pub fn lerp(self, t: Point3) -> Point3 {
        Point3::new(lerp(t.x, self.p_min.x, self.p_max.x), lerp(t.y, self.p_min.y, self.p_max.y), lerp(t.z, self.p_min.z, self.p_max.z))
    }

    pub fn offset(self, p: Point3) -> Vector3 {
        let mut o = p - self.p_min;
        if self.p_max.x > self.p_min.x {
            o.x /= self.p_max.x - self.p_min.x;
        }
        if self.p_max.y > self.p_min.y {
            o.y /= self.p_max.y - self.p_min.y;
        }
        if self.p_max.z > self.p_min.z {
            o.z /= self.p_max.z - self.p_min.z;
        }

        o
    }

    pub fn inside(self, c: &Point3) -> bool {
        self.p_min.x <= c.x && c.x <= self.p_max.x && 
        self.p_min.y <= c.y && c.y <= self.p_max.y && 
        self.p_min.z <= c.z && c.z <= self.p_max.z 
    }

    pub fn bounding_sphere(self, c: &mut Point3, rad: &mut Float) {
        let diag = self.diagonal();
        *c = self.p_min + diag * 0.5;
        *rad = if self.inside(c) {
            diag.norm() / 2.0
        } else {
            0.0
        }
    }

    //TODO - impl this
    pub fn intersect_p(_ray: &Ray, _hit_t0: &mut Float, _hit_t1: &mut Float) -> bool {
        false
    }

    pub fn union_pt(b: &Self, p: &Point3) -> Self {
        Self {
            p_min: Point3::new(
                b.p_min.x.min(p.x), b.p_min.y.min(p.y), b.p_min.z.min(p.z)
            ),
            p_max: Point3::new(
                b.p_max.x.max(p.x), b.p_max.y.max(p.y), b.p_max.z.max(p.z)
            )
        }
    }

    pub fn union(b1: &Self, b2: &Self) -> Self {
        Self {
            p_min: Point3::new(
                b1.p_min.x.min(b2.p_min.x), b1.p_min.y.min(b2.p_min.y), b1.p_min.z.min(b2.p_min.z)
            ),
            p_max: Point3::new(
                b1.p_max.x.max(b2.p_max.x), b1.p_max.y.max(b2.p_max.y), b1.p_max.z.max(b2.p_max.z)
            )
        }
    }

    pub fn intersect(b1: &Self, b2: &Self) -> Self {
        Self {
            p_min: Point3::new(
                b1.p_min.x.max(b2.p_min.x), b1.p_min.y.max(b2.p_min.y), b1.p_min.z.max(b2.p_min.z)
            ),
            p_max: Point3::new(
                b1.p_max.x.min(b2.p_max.x), b1.p_max.y.min(b2.p_max.y), b1.p_max.z.min(b2.p_max.z)
            )
        }
    }

    pub fn overlaps(b1: &Self, b2: &Self) -> bool {
        let x = (b1.p_max.x >= b2.p_min.x) && (b1.p_min.x <= b2.p_max.x);
        let y = (b1.p_max.y >= b2.p_min.y) && (b1.p_min.y <= b2.p_max.y);
        let z = (b1.p_max.z >= b2.p_min.z) && (b1.p_min.z <= b2.p_max.z);

        x && y && z
    }

    pub fn expand(b: &Self, delta: Float) ->  Self {
        Self::init(&(b.p_min - Vector3::new(delta, delta, delta)), &(b.p_max + Vector3::new(delta, delta, delta)))
    }
}

impl Index<usize> for Bounds3f {
    type Output = Point3;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            return &self.p_min
        }

        &self.p_max
    }
}

impl IndexMut<usize> for Bounds3f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                if index == 0 {
            return &mut self.p_min
        }

        &mut self.p_max
    }
}

impl PartialEq for Bounds3f {
    fn eq(&self, other: &Self) -> bool {
        self.p_max == other.p_max && self.p_min == other.p_min
    }

    fn ne(&self, other: &Self) -> bool {
        self.p_max != other.p_max || self.p_min != other.p_min
    }
}