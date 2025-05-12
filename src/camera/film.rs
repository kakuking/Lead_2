use crate::common::*;

const FILTER_TABLE_WIDTH: usize = 16usize;

#[derive(Debug, Clone, Copy)]
pub struct FilmTilePixel {
    contrib_sum: Spectrum,
    filter_weight_sum: Float
}

impl FilmTilePixel {
    pub fn new() -> Self {
        Self {
            contrib_sum: Spectrum::new(0.0, 0.0, 0.0),
            filter_weight_sum: 0.0
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilmTile {
    pub pixel_bounds: Bounds2f,
    pub filter_radius: Vector2,
    pub inv_filter_radius: Vector2,
    pub filter_table: [Float; FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH],
    pub filter_table_size: u32,
    pub pixels: Vec<FilmTilePixel>,
}

impl FilmTile {
    pub fn init(pixel_bounds: Bounds2f, filter_radius: Vector2, filter_table: [Float; FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH], filter_table_size: u32) -> Self {
        let num_pixels = pixel_bounds.area() as usize;
        let pixels = vec![FilmTilePixel::new(); num_pixels];
        let inv_filter_radius = Vector2::new(1.0 / filter_radius.x, 1.0 / filter_radius.y);
        Self {
            pixel_bounds, 
            filter_radius,
            filter_table,
            filter_table_size,
            inv_filter_radius,
            pixels
        }
    }

    pub fn add_sample(&mut self, p_film: &Point2, l: &Spectrum, sample_weight: Float) {
        let p_film_discrete = p_film - Vector2::new(0.5, 0.5);
        let mut p0 = ceil(p_film_discrete - self.filter_radius);
        let mut p1 = floor(p_film_discrete + self.filter_radius) + Vector2::new(1.0, 1.0);

        p0 = max(&p0, &self.pixel_bounds.p_min);
        p1 = min(&p1, &self.pixel_bounds.p_max);

        let mut ifx: Vec<u32> = Vec::new();
        let mut ify: Vec<u32> = Vec::new();
        let p0x = p0.x as usize;
        let p1x = p1.x as usize;
        let p0y = p0.y as usize;
        let p1y = p1.y as usize;

        for x in p0x..p1x {
            let fx = (x as Float - p_film_discrete.x as Float) * self.inv_filter_radius.x * self.filter_table_size as Float;
            ifx[x - p0x] = fx.floor().min(self.filter_table_size as Float - 1.0) as u32;
        }
        for y in p0y..p1y {
            let fy = (y as Float - p_film_discrete.y as Float) * self.inv_filter_radius.y * self.filter_table_size as Float;
            ify[y - p0y] = fy.floor().min(self.filter_table_size as Float - 1.0) as u32;
        }

        for y in p0y..p1y {
            for x in p0x..p1x {
                let offset = ify[y - p0y] * self.filter_table_size + ifx[x - p0x];
                let filter_weight = self.filter_table[offset as usize];

                let pixel: &mut FilmTilePixel = self.get_pixel(&Point2::new(x as f32, y as f32));
                pixel.contrib_sum += l * sample_weight * filter_weight;
                pixel.filter_weight_sum += filter_weight;
            }
        }
    }

    pub fn get_pixel(&mut self, p: &Point2) -> &mut FilmTilePixel {
        let w = self.pixel_bounds.diagonal().x;
        let offset = (p.x - self.pixel_bounds.p_min.x) + (p.y - self.pixel_bounds.p_min.y) * w;

        &mut self.pixels[offset as usize]
    }

    pub fn get_pixel_const(&self, p: &Point2) -> &FilmTilePixel {
        let w = self.pixel_bounds.diagonal().x;
        let offset = (p.x - self.pixel_bounds.p_min.x) + (p.y - self.pixel_bounds.p_min.y) * w;

        &self.pixels[offset as usize]
    }

    pub fn get_pixel_bounds(&self) -> Bounds2f {
        self.pixel_bounds
    }
}

#[derive(Debug, Clone)]
struct Pixel {
    xyz: [Float; 3],
    filter_weight_sum: Float,
    _splat_xyz: [Float; 3]
}

impl Pixel {
    pub fn new() -> Self {
        Self {
            xyz: [0.0; 3],
            filter_weight_sum: 0.0,
            _splat_xyz: [0.0; 3]
        }
    }
}

#[derive(Debug)]
pub struct Film {
    pub full_resolution: Point2,
    pub diagonal: Float,
    pub filter: Arc<dyn Filter>,
    pub filename: String,
    pub cropped_pixel_bounds: Bounds2f,

    pixels: Arc<Mutex<Vec<Pixel>>>,
    filter_table: [Float; FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH],
    _scale: Float
}

impl Film {
    pub fn new() -> Self {
        Self {
            full_resolution: Point2::new(0.0, 0.0),
            diagonal: 0.0,
            filter: Arc::from(BoxFilter::init(Vector2::new(0.0, 0.0))),
            filename: String::from(""),
            cropped_pixel_bounds: Bounds2f::new(),

            pixels: Arc::from(Mutex::from(Vec::<Pixel>::new())),
            filter_table: [0.0; FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH],
            _scale: 0.0,
        }
    }

    pub fn init(resolution: Point2, crop_window: Bounds2f, filter: Arc<dyn Filter>, diagonal: Float, filename: String, scale: Float) -> Self {
        let full_resolution = resolution;
        let diagonal = diagonal * 0.001;

        let cropped_pmin = Point2::new((full_resolution.x * crop_window.p_min.x).ceil(), (full_resolution.y * crop_window.p_min.y).ceil());
        let cropped_pmax = Point2::new((full_resolution.x * crop_window.p_max.x).ceil(), (full_resolution.y * crop_window.p_max.y).ceil());
        let cropped_pixel_bounds = Bounds2f::init(&cropped_pmin, &cropped_pmax);

        let num_pixels = cropped_pixel_bounds.area() as usize;
        let pixels = Arc::from(Mutex::from(vec![Pixel::new(); num_pixels]));

        let mut offset = 0usize;
        let mut filter_table = [0.0; FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH];

        for y in 0..FILTER_TABLE_WIDTH {
            for x in 0..FILTER_TABLE_WIDTH {
                let mut p = Point2::new(0.0, 0.0);
                p.x = (x as Float + 0.5) * filter.radius().x / FILTER_TABLE_WIDTH as Float;
                p.y = (y as Float + 0.5) * filter.radius().y / FILTER_TABLE_WIDTH as Float;
                filter_table[offset] = filter.evaluate(&p);
                offset += 1;
            }
        }

        Self {
            full_resolution,
            diagonal,
            filter: filter,
            filename,
            cropped_pixel_bounds,
            pixels,
            filter_table,
            _scale: scale
        }
    }

    pub fn get_sample_bounds(&self) -> Bounds2f {
        let p_min = Point2::new(self.cropped_pixel_bounds.p_min.x, self.cropped_pixel_bounds.p_min.y) + Vector2::new(0.5, 0.5) - self.filter.radius();
        let p_max = Point2::new(self.cropped_pixel_bounds.p_max.x, self.cropped_pixel_bounds.p_max.y) - Vector2::new(0.5, 0.5)+ self.filter.radius();

        let p_min_floor = Point2::new(p_min.x.floor(), p_min.y.floor());
        let p_max_ceil = Point2::new(p_max.x.ceil(), p_max.y.ceil());

        Bounds2f::init(&p_min_floor, &p_max_ceil)
    }

    pub fn get_physical_extent(&self) -> Bounds2f {
        let aspect = self.full_resolution.y / self.full_resolution.x;
        let x = (self.diagonal * self.diagonal / (1.0 + aspect * aspect)).sqrt();
        let y = aspect * x;

        let p_min = Point2::new(-x/2.0, -y/2.0);
        let p_max = Point2::new(x/2.0, y/2.0);

        Bounds2f::init(&p_min, &p_max)
    }

    pub fn get_film_tile(&mut self, sample_bounds: Bounds2f) -> Arc<Mutex<FilmTile>> {
        let half_pixel = Vector2::new(0.5, 0.5);
        let p0 = ceil(sample_bounds.p_min - half_pixel - self.filter.radius());
        let p1 = floor(sample_bounds.p_max - half_pixel + self.filter.radius()) + Vector2::new(1.0, 1.0);

        let tile_pixel_bounds = Bounds2f::intersect(&Bounds2f::init(&p0, &p1), &self.cropped_pixel_bounds);

        Arc::from(Mutex::from(FilmTile::init(tile_pixel_bounds, self.filter.radius(), self.filter_table, (FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH) as u32)))
    }

    pub fn merge_film_tile(&mut self, tile: Arc<Mutex<FilmTile>>) {
        let mut tile = tile.lock().unwrap();
        let mut pixels = self.pixels.lock().unwrap();

        let pix_x_min = tile.pixel_bounds.p_min.x as usize;
        let pix_x_max = tile.pixel_bounds.p_max.x as usize;
        let pix_y_min = tile.pixel_bounds.p_min.y as usize;
        let pix_y_max = tile.pixel_bounds.p_max.y as usize;
        for x in pix_x_min..pix_x_max {
            for y in pix_y_min..pix_y_max {
                let pixel = Point2::new(x as f32, y as f32);

                let tile_pixel = tile.get_pixel(&pixel);
                let merge_pixel = &mut pixels[self.get_pixel_offset(pixel)];
                let mut x_contrib: Float = 0.0;
                let mut y_contrib: Float = 0.0;
                let mut z_contrib: Float = 0.0;
                to_xyz(&tile_pixel.contrib_sum, &mut x_contrib, &mut y_contrib, &mut z_contrib);
                merge_pixel.xyz[0] += x_contrib;
                merge_pixel.xyz[1] += y_contrib;
                merge_pixel.xyz[2] += z_contrib;
                merge_pixel.filter_weight_sum = tile_pixel.filter_weight_sum;
            }
        }
    }

    pub fn set_image(&self, _image: Vec<Spectrum>) {
        todo!("TO DO")
    }

    pub fn add_splat(&mut self, _p: &Point2, _v: &Spectrum) {
        todo!("TO DO")
    }

    pub fn write_image(&self, _splat_scale: Float) {
        todo!("TO DO")
    }

    pub fn clear(&mut self) {
        todo!("TO DO")
    }

    fn get_pixel_offset(&self, p: Point2) -> usize {
        let width = (self.cropped_pixel_bounds.p_max.x - self.cropped_pixel_bounds.p_min.x) as usize;
        let offset = (p.x - self.cropped_pixel_bounds.p_min.x) as usize + (p.y - self.cropped_pixel_bounds.p_min.y) as usize * width;

        offset
    }
}