pub use crate::common::*;

pub trait ProjectiveCamera: Camera {
    fn camera_to_screen(&self) -> Arc<Transform>;
    fn raster_to_camera(&self) -> Arc<Transform>;
    fn screen_to_raster(&self) -> Arc<Transform>;
    fn raster_to_screen(&self) -> Arc<Transform>;
    fn lens_radius(&self) -> Float;
    fn focal_distance(&self) -> Float;

    fn set_lens_radius(&mut self, lr: Float);
    fn set_focal_distance(&mut self, lr: Float);
    fn set_camera_to_screen(&mut self, new_val: Arc<Transform>);
    fn set_raster_to_camera(&mut self, new_val: Arc<Transform>);
    fn set_screen_to_raster(&mut self, new_val: Arc<Transform>);
    fn set_raster_to_screen(&mut self, new_val: Arc<Transform>);
    
    fn init(&mut self, camera_to_world: Arc<Transform>, camera_to_screen: Arc<Transform>, screen_window: Bounds2f, shutter_open: Float, shutter_close: Float, lens_r: Float, focald: Float, film: Arc<Film>, medium: Option<Arc<dyn Medium>>) {
        Camera::init(self, camera_to_world, shutter_open, shutter_close, film.clone(), medium);

        let mut screen_to_raster = scale(&Vector3::new(film.full_resolution.x, film.full_resolution.y, 1.0));

        let delta = screen_window.p_max - screen_window.p_min;
        screen_to_raster = screen_to_raster * scale(&Vector3::new(1.0/delta.x, 1.0/ delta.y, 1.0));
        screen_to_raster = screen_to_raster * translate(&Vector3::new(-screen_window.p_min.x, -screen_window.p_max.y, 0.0));
        let raster_to_screen = screen_to_raster.inverse();

        let raster_to_camera = camera_to_screen.inverse() * raster_to_screen;

        self.set_lens_radius(lens_r);
        self.set_focal_distance(focald);

        self.set_screen_to_raster(Arc::from(screen_to_raster));
        self.set_raster_to_screen(Arc::from(raster_to_screen));
        self.set_raster_to_camera(Arc::from(raster_to_camera));
        self.set_camera_to_screen(camera_to_screen);

    }
}