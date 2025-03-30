use std::f32;

use crate::common::*;

pub fn lerp(t: Float, v1: Float, v2: Float) -> Float {
    (1.0 - t) * v1 + t * v2
}

pub fn gamma(n: Float) ->  Float {
    return (n * EPSILON) / (1.0 - n * EPSILON);
}

pub fn next_float_up(v: f32) -> f32 {
    // Handle infinity and negative zero
    if v.is_infinite() && v > 0.0 {
        return v;
    }
    if v == -0.0 {
        return 0.0;
    }

    // Advance v to next higher float by manipulating bits
    let bits = v.to_bits();
    let next_bits = if v >= 0.0 {
        bits + 1
    } else {
        bits - 1
    };
    f32::from_bits(next_bits)
}

/// Returns the next representable floating-point number less than `v`.
pub fn next_float_down(v: f32) -> f32 {
    -next_float_up(-v)
}

pub fn quadratic(a: Float, b: Float, c: Float, t0: &mut Float, t1: &mut Float) -> bool {
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        return false;
    }

    let d_rt = d.sqrt();

    let r0 = 0.5 * (-b - d_rt) / a;
    let r1 = 0.5 * (-b + d_rt) / a;

    if r0 > r1 {
        *t0 = r1;  
        *t1 = r0;
    } else {
        *t0 = r0;
        *t1 = r1;
    }

    true
}

pub const PI: Float = f32::consts::PI;