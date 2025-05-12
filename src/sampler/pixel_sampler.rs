use crate::common::*;

#[derive(Debug, Clone)]
pub struct PixelSampler {
    array_1d_offset: usize,
    array_2d_offset: usize,
    current_pixel: Point2,
    current_pixel_sample_idx: usize,
    samples_1d_array_sizes: Vec<usize>,
    samples_2d_array_sizes: Vec<usize>,
    sample_array_1d: Vec<Vec<Float>>,
    sample_array_2d: Vec<Vec<Point2>>,
    samples_per_pixel: usize,

    samples_1d: Vec<Vec<Float>>,
    samples_2d: Vec<Vec<Point2>>,
    current_1d_dimension: usize,
    current_2d_dimension: usize,
    rng: RNG
}

impl PixelSampler {
    pub fn init(samples_per_pixel: usize, n_sampled_dimensions: usize) -> Self {
        let array_1d_offset: usize = 0usize;
        let array_2d_offset: usize = 0usize;
        let current_pixel: Point2 = Point2::new(0.0, 0.0);
        let current_pixel_sample_idx: usize = 0usize;
        let samples_1d_array_sizes: Vec<usize> = Vec::new();
        let samples_2d_array_sizes: Vec<usize> = Vec::new();
        let sample_array_1d: Vec<Vec<Float>> = Vec::new();
        let sample_array_2d: Vec<Vec<Point2>> = Vec::new();
        let samples_per_pixel: usize = samples_per_pixel;

        let samples_1d: Vec<Vec<Float>> = vec![vec![0f32; samples_per_pixel]; n_sampled_dimensions];
        let samples_2d: Vec<Vec<Point2>> = vec![vec![Point2::new(0.0, 0.0); samples_per_pixel]; n_sampled_dimensions];

        let rng = RNG::init(0u64);

        Self {
            array_1d_offset,
            array_2d_offset,
            current_pixel,
            current_pixel_sample_idx,
            samples_1d_array_sizes,
            samples_2d_array_sizes,
            sample_array_1d,
            sample_array_2d,
            samples_per_pixel,

            samples_1d,
            samples_2d,
            current_1d_dimension: 0usize,
            current_2d_dimension: 0usize,
            rng,
        }
    }
}

impl Sampler for PixelSampler {
    fn array_1d_offset(&self) -> usize { self.array_1d_offset }
    fn array_2d_offset(&self) -> usize { self.array_2d_offset }
    fn current_pixel(&self) -> Point2 { self.current_pixel }
    fn current_pixel_sample_idx(&self) -> usize { self.current_pixel_sample_idx }
    fn samples_1d_array_sizes(&mut self) -> &mut Vec<usize> { &mut self.samples_1d_array_sizes }
    fn samples_2d_array_sizes(&mut self) -> &mut Vec<usize> { &mut self.samples_2d_array_sizes }
    fn sample_array_1d(&mut self) -> &mut Vec<Vec<Float>> { &mut self.sample_array_1d }
    fn sample_array_2d(&mut self) -> &mut Vec<Vec<Point2>> { &mut self.sample_array_2d }
    fn samples_per_pixel(&self) -> usize { self.samples_per_pixel }

    fn set_array_1d_offset(&mut self, other: usize) { self.array_1d_offset = other; }
    fn set_array_2d_offset(&mut self, other: usize) { self.array_2d_offset = other; }
    fn set_current_pixel(&mut self, other: Point2) { self.current_pixel = other; }
    fn set_current_pixel_sample_idx(&mut self, other: usize) { self.current_pixel_sample_idx = other; }
    fn set_samples_per_pixel(&mut self, other: usize) { self.samples_per_pixel = other; }

    fn get_1d(&mut self) -> Float {
        if self.current_1d_dimension < self.samples_1d.len() {
            self.current_1d_dimension += 1;
            return self.samples_1d[self.current_1d_dimension - 1][self.current_pixel_sample_idx];
        }

        self.rng.uniform_float()
    }

    fn get_2d(&mut self) -> Point2 {
        if self.current_2d_dimension < self.samples_2d.len() {
            self.current_2d_dimension += 1;
            return self.samples_2d[self.current_2d_dimension - 1][self.current_pixel_sample_idx];
        }

        Point2::new(self.rng.uniform_float(), self.rng.uniform_float())
    }

    // fn clone(&self, seed: usize) -> Arc<Self> {
    //     todo!("")
    // }

    fn start_next_sample(&mut self) -> bool {
        self.current_1d_dimension = 0usize;
        self.current_2d_dimension = 0usize;

        Sampler::default_start_next_sample(self)
    }

    fn set_sample_number(&mut self, sample_num: usize) -> bool {
        self.current_1d_dimension = 0usize;
        self.current_2d_dimension = 0usize;

        Sampler::default_set_sample_number(self, sample_num)
    }
}