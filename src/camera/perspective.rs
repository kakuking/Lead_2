use crate::common::*;

#[derive(Debug)]
pub struct PerspectiveCamera {
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
    a: Float,
}

impl PerspectiveCamera {
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
            a: 0.0
        }
    }

    pub fn init(camera_to_world: Arc<Transform>, screen_window: Bounds2f, shutter_open: Float, shutter_close: Float, lens_radius: Float, focal_distance: Float, fov: Float, film: Arc<Film>, medium: Option<Arc<dyn Medium>>) -> Self {
        let mut ret = Self::new();
        
        let camera_to_screen = Self::create_perspective(fov, 0.01, 1000.0);
        ProjectiveCamera::init(&mut ret, camera_to_world, Arc::from(camera_to_screen), screen_window, shutter_open, shutter_close, lens_radius, focal_distance, film, medium);

        let origin = ret.raster_to_camera.transform_point(&Point3::new(0.0, 0.0, 0.0));
        let dx_camera = ret.raster_to_camera.transform_point(&Point3::new(1.0, 0.0, 0.0));
        let dy_camera = ret.raster_to_camera.transform_point(&Point3::new(0.0, 1.0, 0.0));
        
        ret.dx_camera = dx_camera - origin;
        ret.dy_camera = dy_camera - origin;
        
        let res = ret.film.full_resolution;
        let mut p_min = ret.raster_to_camera.transform_point(&Point3::new(0.0, 0.0, 0.0));
        let mut p_max = ret.raster_to_camera.transform_point(&Point3::new(res.x, res.y, 0.0));
        p_min = p_min / p_min.z;
        p_max = p_max / p_max.z;

        let a = (p_max.x - p_min.x) * (p_max.y - p_min.y);
        ret.a = a.abs();

        ret
    }

    fn create_perspective(fov: Float, n: Float, f: Float) -> Transform {
        let z_scale = f / (f - n);
        let z_translate = -f * n /(f - n);

        let scaling = scale(&Vector3::new(1.0, 1.0, z_scale));
        let translation = translate(&Vector3::new(0.0, 0.0, z_translate));
        
        let inv_tan_angle = 1.0 / (fov.to_radians() / 2.0).tan();
        let other_scaling = scale(&Vector3::new(inv_tan_angle, inv_tan_angle, 1.0));
        other_scaling * scaling * translation
    }
}

impl Camera for PerspectiveCamera {
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

    fn generate_ray(&self, _sample: &camera::CameraSample, _r: &mut Ray) -> Float {
        todo!("In 6.2.2")
    }

    fn generate_ray_differential(&self, _sample: &camera::CameraSample, _r: &mut RayDifferential) -> Float {
        todo!("In 6.2.2")
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

impl ProjectiveCamera for PerspectiveCamera {
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