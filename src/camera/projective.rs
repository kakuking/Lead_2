pub use crate::common::*;

pub trait ProjectiveCamera: Camera {
    fn camera_to_screen(&self) -> Arc<Transform>;
    fn screen_to_camera(&self) -> Arc<Transform>;
    fn screen_to_raster(&self) -> Arc<Transform>;
    fn raster_to_screen(&self) -> Arc<Transform>;
    fn lens_radius(&self) -> Float;
    fn focal_distance(&self) -> Float;

    
}