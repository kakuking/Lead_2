use crate::common::*;

#[derive(Debug, Clone, Copy)]
pub struct CameraSample {
    pub p_film: Point2,
    pub p_lens: Point2,
    pub time: Float
}

pub trait Camera: Debug {
    fn camera_to_world(&self) -> Arc<Transform>;
    fn shutter_open(&self) -> Float;
    fn shutter_close(&self) -> Float;
    fn medium(&self) -> Option<Arc<dyn Medium>>;
    fn film(&self) -> Arc<Film>;
    
    fn set_camera_to_world(&mut self, other: Arc<Transform>);
    fn set_shutter_open(&mut self, other: Float);
    fn set_shutter_close(&mut self, other: Float);
    fn set_medium(&mut self, other: Option<Arc<dyn Medium>>);
    fn set_film(&mut self, other: Arc<Film>);

    fn init(&mut self, camera_to_world: Arc<Transform>, shutter_open: Float, shutter_close: Float, film: Arc<Film>, medium: Option<Arc<dyn Medium>>) {
        self.set_camera_to_world(camera_to_world);
        self.set_shutter_open(shutter_open);
        self.set_shutter_close(shutter_close);
        self.set_film(film);
        self.set_medium(medium);
    }

    fn generate_ray(&self, sample: &CameraSample, r: &mut Ray) -> Float;
    fn generate_ray_differential(&self, sample: &CameraSample, rd: &mut RayDifferential) -> Float {
        let mut ray = Ray::new();
        let wt = self.generate_ray(sample, &mut ray);

        let mut sshift = sample.clone();
        sshift.p_film.x += 1.0;
        let mut rx = Ray::new();
        let wtx = self.generate_ray(&sshift, &mut rx);
        if wtx == 0.0 {
            return 0.0;
        }

        sshift.p_film.x -= 1.0;
        sshift.p_film.y += 1.0;
        let mut ry = Ray::new();
        let wty = self.generate_ray(&sshift, &mut ry);
        if wty == 0.0 {
            return 0.0;
        }

        *rd = RayDifferential {
            ray,
            has_differentials: true,
            rx_origin: rx.o,
            rx_direction: rx.d,
            ry_origin: ry.o,
            ry_direction: ry.d
        };

        wt
    }

    fn we(&self, p_raster_2: Vec<Point2>) -> Spectrum;
    fn pdf_we(&self, ray: &Ray, pdf_pos: &mut Float, pdf_dir: &mut Float);

    fn sample_wi(&self, reference: &Interaction, u: &Point2, wi: &mut Vector3, pdf: &mut Float, p_raster: Vec<Point2>, visibility_tester: &mut VisibilityTester) -> Spectrum;
}