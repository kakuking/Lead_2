use crate::common::*;

use std::ops::{Index, IndexMut};
use std::cmp::PartialEq;

#[derive(Debug, Clone, Copy)]
pub struct Bounds2f {
    pub p_min: Point2,
    pub p_max: Point2
}

impl Bounds2f {
    pub fn new() -> Self {
        let min_num: Float = -INFINITY;
        let max_num: Float = INFINITY;

        Self {
            p_min: Point2::new(min_num, min_num),
            p_max: Point2::new(max_num, max_num)
        }
    }

    pub fn init(p1: &Point2, p2: &Point2) -> Self {
        Self{
            p_min: Point2::new(p1.x.min(p2.x), p1.y.min(p2.y)),
            p_max: Point2::new(p1.x.max(p2.x), p1.y.max(p2.y)),
        }
    }

    pub fn init_one(p: &Point2) -> Self {
        Self{
            p_min: p.clone(),
            p_max: p.clone()
        }
    }

    pub fn diagonal(&self) -> Vector2 {
        self.p_max - self.p_min
    }

    pub fn area(&self) -> Float {
        let diag = self.diagonal();
        diag.x * diag.y
    }
    // Which is longer, x or y
    pub fn max_extent(&self) -> usize {
        let diag = self.diagonal();

        if diag.x > diag.y {
            return 0;
        }

        1
    }

    pub fn lerp(&self, t: Point2) -> Point2 {
        Point2::new(lerp(t.x, self.p_min.x, self.p_max.x), lerp(t.y, self.p_min.y, self.p_max.y))
    }

    pub fn offset(&self, p: &Point2) -> Vector2 {
        let mut o = p - self.p_min;
        if self.p_max.x > self.p_min.x {
            o.x /= self.p_max.x - self.p_min.x;
        }
        if self.p_max.y > self.p_min.y {
            o.y /= self.p_max.y - self.p_min.y;
        }

        o
    }

    pub fn inside(&self, c: &Point2) -> bool {
        self.p_min.x <= c.x && c.x <= self.p_max.x && 
        self.p_min.y <= c.y && c.y <= self.p_max.y 
    }

    pub fn bounding_sphere(&self, c: &mut Point2, rad: &mut Float) {
        let diag = self.diagonal();
        *c = self.p_min + diag * 0.5;
        *rad = if self.inside(c) {
            diag.norm() / 2.0
        } else {
            0.0
        }
    }
}

impl Index<usize> for Bounds2f {
    type Output = Point2;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            return &self.p_min
        }

        &self.p_max
    }
}

impl IndexMut<usize> for Bounds2f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                if index == 0 {
            return &mut self.p_min
        }

        &mut self.p_max
    }
}

impl PartialEq for Bounds2f {
    fn eq(&self, other: &Self) -> bool {
        self.p_max == other.p_max && self.p_min == other.p_min
    }

    fn ne(&self, other: &Self) -> bool {
        self.p_max != other.p_max || self.p_min != other.p_min
    }
}