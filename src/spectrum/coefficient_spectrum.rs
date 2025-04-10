use crate::common::*;

// pub const N_SPECTRUM_SAMPLES 

// pub trait CoefficientSpectrum<const N_SPECTRUM_SAMPLES: usize>: Debug + Copy + Clone {

// }

pub type RBGSpectrum = na::Vector3<Float>;

pub fn from_rgb(r: Float, g: Float, b: Float) -> RBGSpectrum {
    RBGSpectrum::new(r, g, b)
}

pub fn to_rgb(rgb: &RBGSpectrum, r: &mut Float, g: &mut Float, b: &mut Float) {
    *r = rgb.x;
    *g = rgb.y;
    *b = rgb.z;
}

pub fn to_xyz(rgb: &RBGSpectrum, x: &mut Float, y: &mut Float, z: &mut Float) {
    *x = 0.412453*rgb[0] + 0.357580*rgb[1] + 0.180423*rgb[2];
    *y = 0.212671*rgb[0] + 0.715160*rgb[1] + 0.072169*rgb[2];
    *z = 0.019334*rgb[0] + 0.119193*rgb[1] + 0.950227*rgb[2];
}

pub fn from_xyz(x: Float, y: Float, z: Float) -> RBGSpectrum {
    let r = 0.412453*x + 0.357580*y + 0.180423*z;
    let g = 0.212671*x + 0.715160*y + 0.072169*z;
    let b = 0.019334*x + 0.119193*y + 0.950227*z;

    from_rgb(r, g, b)
}

pub fn rgb_y(rgb: &RBGSpectrum) -> Float {
    0.212671 * rgb[0] + 0.715160 * rgb[1] + 0.072169 * rgb[2]
}