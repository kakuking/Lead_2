use std::fmt::Debug;

// use crate::common::*;

pub trait Shape: Debug {
    fn reverse_orientation(&self) -> bool;
    fn transform_swaps_handedness(&self) -> bool;
}

pub trait Primitive: Debug {
    
} 