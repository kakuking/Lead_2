use crate::common::*;

#[derive(Debug)]
pub struct OrthographicCamera {
    camera_to_world: Arc<Transform>,
    camera_to_screen: Arc<Transform>,
    raster_to_camera: Arc<Transform>,
    screen_to_raster: Arc<Transform>,
    raster_to_screen: Arc<Transform>,

    lens_radius: Float,
    focal_distance: Float,

    shutter_open: Float,
    shutter_close: Float,
    medium: Option<Arc<dyn Medium>>,
    film: Arc<Film>,

    dx_camera: Vector3,
    dy_camera: Vector3,
}

impl OrthographicCamera {
    pub fn new() -> Self {
        let iden = Arc::from(Transform::identity());
        let new_film = Arc::from(Film::new());

        Self {
            camera_to_world: iden.clone(),
            camera_to_screen: iden.clone(),
            raster_to_camera: iden.clone(),
            screen_to_raster: iden.clone(),
            raster_to_screen: iden.clone(),

            lens_radius: 0.0,
            focal_distance: 0.0,

            shutter_open: 0.0,
            shutter_close: 0.0,
            medium: None,
            film: new_film,

            dx_camera: Vector3::new(0.0, 0.0, 0.0),
            dy_camera: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn init(camera_to_world: Arc<Transform>, screen_window: Bounds2f, shutter_open: Float, shutter_close: Float, lens_radius: Float, focal_distance: Float, film: Arc<Film>, medium: Option<Arc<dyn Medium>>) -> Self {
        let mut ret = Self::new();

        let camera_to_screen = Self::creat_orthographic(0.0, 1.0);
        ProjectiveCamera::init(&mut ret, camera_to_world, Arc::from(camera_to_screen), screen_window, shutter_open, shutter_close, lens_radius, focal_distance, film, medium);

        ret.dx_camera = ret.raster_to_camera.transform_vector(&Vector3::new(1.0, 0.0, 0.0));
        ret.dy_camera = ret.raster_to_camera.transform_vector(&Vector3::new(0.0, 1.0, 0.0));

        ret
    }

    fn creat_orthographic(z_near: Float, z_far: Float) -> Transform {
        let scaling = scale(&Vector3::new(1.0, 1.0, 1.0/(z_far - z_near)));
        let translation = translate(&Vector3::new(0.0, 0.0, -z_near));

        scaling * translation
    }
}

impl Camera for OrthographicCamera {
    fn camera_to_world(&self) -> Arc<Transform> { self.camera_to_world.clone() }
    fn shutter_open(&self) -> Float { self.shutter_open }
    fn shutter_close(&self) -> Float { self.shutter_close }
    fn medium(&self) -> Option<Arc<dyn Medium>> { self.medium.clone() }
    fn film(&self) -> Arc<Film> { self.film.clone() }

    fn set_camera_to_world(&mut self, other: Arc<Transform>) { self.camera_to_world = other; }
    fn set_shutter_open(&mut self, other: Float) { self.shutter_open = other; }
    fn set_shutter_close(&mut self, other: Float) { self.shutter_close = other; }
    fn set_medium(&mut self, other: Option<Arc<dyn Medium>>) { self.medium = other; }
    fn set_film(&mut self, other: Arc<Film>) { self.film = other; }

    fn generate_ray(&self, sample: &camera::CameraSample, r: &mut Ray) -> Float {
        let p_film = Point3::new(sample.p_film.x, sample.p_film.y, 0.0);
        let p_camera = self.raster_to_camera.transform_vector(&p_film.coords);
        let p_camera = Point3::new(p_camera.x, p_camera.y, p_camera.z);

        *r = Ray::init(&p_camera, &Vector3::new(0.0, 0.0, 1.0), Some(INFINITY), Some(0.0), self.medium().clone());

        if self.lens_radius > 0.0 {
            let p_lens = self.lens_radius * sample_concentric_disc(&sample.p_lens);

            let ft = self.focal_distance / r.d.z;
            let p_focus = r.at(ft);

            (*r).o = Point3::new(p_lens.x, p_lens.y, 0.0);
            (*r).d = (p_focus - r.o).normalize()
        }

        (*r).time = lerp(sample.time, self.shutter_open, self.shutter_close);
        *r = apply_transform_to_ray(&r, &self.camera_to_world);

        1.0
    }

    fn generate_ray_differential(&self, _sample: &camera::CameraSample, _rd: &mut RayDifferential) -> Float {
        todo!("Implement in 6.2.1")
    }
    
    fn we(&self, _p_raster_2: Vec<Point2>) -> Spectrum {
        todo!("Not implemented yet")
    }

    fn pdf_we(&self, _ray: &Ray, _pdf_pos: &mut Float, _pdf_dir: &mut Float) {
        todo!("Not implemented yet")
    }

    fn sample_wi(&self, _reference: &Interaction, _u: &Point2, _wi: &mut Vector3, _pdf: &mut Float, _p_raster: Vec<Point2>, _visibility_tester: &mut VisibilityTester) -> Spectrum {
        todo!("Not implemented yet")
    }
}

impl ProjectiveCamera for OrthographicCamera {
    fn camera_to_screen(&self) -> Arc<Transform> { self.camera_to_screen.clone() }
    fn raster_to_camera(&self) -> Arc<Transform> { self.raster_to_camera.clone() }
    fn screen_to_raster(&self) -> Arc<Transform> { self.screen_to_raster.clone() }
    fn raster_to_screen(&self) -> Arc<Transform> { self.raster_to_screen.clone() }
    fn lens_radius(&self) -> Float { self.lens_radius }
    fn focal_distance(&self) -> Float { self.focal_distance }

    fn set_camera_to_screen(&mut self, new_val: Arc<Transform>) { self.camera_to_screen = new_val; }
    fn set_raster_to_camera(&mut self, new_val: Arc<Transform>) { self.raster_to_camera = new_val; }
    fn set_screen_to_raster(&mut self, new_val: Arc<Transform>) { self.screen_to_raster = new_val; }
    fn set_raster_to_screen(&mut self, new_val: Arc<Transform>) { self.raster_to_screen = new_val; }
    fn set_lens_radius(&mut self, new_val: Float) { self.lens_radius = new_val; }
    fn set_focal_distance(&mut self, new_val: Float) { self.focal_distance = new_val; }
}